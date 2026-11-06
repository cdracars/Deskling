// SPDX-License-Identifier: MIT
//! Layered 2D rig renderer. Source layers, parent joints and keyframes are authoring
//! data only; runtime output remains Deskling's strict schema-1 PNG/JSON pack.
use super::{json, obj, num, text, new_output, parse_project, read_json, measure, warn, Audit, Json, Image, Pack, Region, Result};
use super::png;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pose { dx: f64, dy: f64, angle: f64, scale: f64 }
impl Default for Pose {
    fn default() -> Self { Self { dx: 0.0, dy: 0.0, angle: 0.0, scale: 1.0 } }
}
#[derive(Clone, Copy, Debug)]
struct Mat { a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64 }
impl Mat {
    fn mul(self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            tx: self.a * other.tx + self.c * other.ty + self.tx,
            ty: self.b * other.tx + self.d * other.ty + self.ty,
        }
    }
    fn point(&self, x: f64, y: f64) -> (f64, f64) {
        (self.a * x + self.c * y + self.tx, self.b * x + self.d * y + self.ty)
    }
}
#[derive(Clone)]
struct Part {
    id: String,
    image: Image,
    parent: Option<usize>,
    pivot: (f64, f64),
    at: (f64, f64),
}
#[derive(Clone)]
struct Keyframe { frame: u32, poses: BTreeMap<String, Pose> }
struct Rig {
    native: Json,
    pack: Pack,
    parts: Vec<Part>,
    clips: HashMap<String, Vec<Keyframe>>,
    locks: BTreeMap<String, Vec<Region>>,
}
fn range(v: &Json, field: &str, low: f64, high: f64) -> Result<f64> {
    match v {
        Json::Number(n) if n.is_finite() && *n >= low && *n <= high => Ok(*n),
        _ => Err(format!("{field}: expected finite number between {low} and {high}")),
    }
}
fn coords(v: &Json, field: &str, low: f64, high: f64) -> Result<(f64, f64)> {
    let a = v.array()?;
    if a.len() != 2 { return Err(format!("{field}: expected [x,y]")); }
    Ok((range(&a[0], field, low, high)?, range(&a[1], field, low, high)?))
}
fn label(map: &BTreeMap<String, Json>, key: &str) -> Result<String> {
    let s = json::field(map, key)?.string()?;
    if !super::slug(s) { return Err(format!("{key}: must be a lowercase slug")); }
    Ok(s.into())
}
fn pose(value: &Json) -> Result<Pose> {
    let obj = value.object()?;
    json::only(obj, &["dx", "dy", "rotate", "scale"])?;
    let mut out = Pose::default();
    if let Some(v) = obj.get("dx") { out.dx = range(v, "dx", -1024.0, 1024.0)?; }
    if let Some(v) = obj.get("dy") { out.dy = range(v, "dy", -1024.0, 1024.0)?; }
    if let Some(v) = obj.get("rotate") { out.angle = range(v, "rotate", -360.0, 360.0)?; }
    if let Some(v) = obj.get("scale") { out.scale = range(v, "scale", 0.25, 4.0)?; }
    Ok(out)
}
fn clip_value(v: &Json, part_ids: &[String], frames: u32) -> Result<Vec<Keyframe>> {
    let entries = v.array()?;
    if entries.is_empty() { return Err("clip must contain at least one keyframe".into()); }
    let mut clip = Vec::<Keyframe>::new();
    for entry in entries {
        let e = entry.object()?;
        json::only(e, &["frame", "poses"])?;
        let index = json::field(e, "frame")?.integer(0, frames - 1)?;
        if let Some(prev) = clip.last() {
            if prev.frame >= index { return Err("clip keyframes must be in strictly increasing order".into()); }
        }
        let mut transforms = BTreeMap::new();
        for (id, value) in json::field(e, "poses")?.object()? {
            if !part_ids.contains(id) { return Err(format!("clip references unknown part '{id}'")); }
            transforms.insert(id.clone(), pose(value)?);
        }
        clip.push(Keyframe { frame: index, poses: transforms });
    }
    if clip[0].frame != 0 || clip.last().unwrap().frame != frames - 1 {
        return Err(format!("clip must explicitly define frame 0 and frame {}", frames - 1));
    }
    Ok(clip)
}
fn layer_path(dir: &Path, name: &str) -> Result<PathBuf> {
    if name.is_empty() || name.contains(['/', '\\', ':']) || name.starts_with('.') ||
        !name.to_ascii_lowercase().ends_with(".png") {
        return Err("layer image must be a local PNG basename".into());
    }
    let layers = dir.join("layers").canonicalize().map_err(|e| format!("layers folder: {e}"))?;
    let file = layers.join(name).canonicalize().map_err(|e| format!("layer '{name}': {e}"))?;
    if file.parent() != Some(layers.as_path()) {
        return Err(format!("layer '{name}' resolves outside layers folder"));
    }
    Ok(file)
}
fn read_rig(path: &Path) -> Result<Rig> {
    let root = read_json(path)?;
    let map = root.object()?;
    json::only(map, &["schemaVersion", "pack", "parts", "motions", "lockedRegions"])?;
    json::field(map, "schemaVersion")?.integer(1, 1)?;
    let native = json::field(map, "pack")?.clone();
    let mut p = vec![("schemaVersion", num(1)), ("pack", native.clone())];
    if let Some(locks) = map.get("lockedRegions") { p.push(("lockedRegions", locks.clone())); }
    let (_, pack, locks) = parse_project(&obj(p))?;
    let dir = path.canonicalize().map_err(|e| e.to_string())?.parent()
        .ok_or("rig has no parent directory")?.to_path_buf();
    let raw_parts = json::field(map, "parts")?.array()?;
    if raw_parts.is_empty() || raw_parts.len() > 64 { return Err("rig requires 1-64 parts".into()); }
    let mut ids = Vec::<String>::new();
    for part in raw_parts {
        let m = part.object()?;
        json::only(m, &["id", "image", "parent", "pivot", "at"])?;
        let id = label(m, "id")?;
        if ids.contains(&id) { return Err(format!("duplicate part '{id}'")); }
        ids.push(id);
    }
    let mut parts = Vec::new();
    for (i, part) in raw_parts.iter().enumerate() {
        let m = part.object()?;
        let basename = json::field(m, "image")?.string()?;
        let bytes = fs::read(layer_path(&dir, basename)?).map_err(|e| e.to_string())?;
        let image = png::decode(&bytes).map_err(|e| format!("part '{}': {e}", ids[i]))?;
        // Layers are allowed to be smaller or larger than their parent joint geometry,
        // but are bounded by the codec's decoded-pixel limit.
        let pivot = coords(json::field(m, "pivot")?, "pivot", 0.0, 16383.0)?;
        if pivot.0 > (image.width - 1) as f64 || pivot.1 > (image.height - 1) as f64 {
            return Err(format!("pivot outside image for part '{}'", ids[i]));
        }
        let at = coords(json::field(m, "at")?, "at", -2048.0, 2048.0)?;
        let parent = match m.get("parent") {
            None | Some(Json::Null) => None,
            Some(v) => Some(ids.iter().position(|x| x == v.string().unwrap_or(""))
                .ok_or_else(|| format!("unknown parent for part '{}'", ids[i]))?),
        };
        if parent == Some(i) { return Err(format!("part '{}' cannot parent itself", ids[i])); }
        parts.push(Part { id: ids[i].clone(), image, parent, pivot, at });
    }
    // Parent ancestry is independent of draw order. Reject cycles, including
    // cycles that do not involve the first part.
    for i in 0..parts.len() {
        let mut seen = vec![false; parts.len()];
        let mut current = Some(i);
        while let Some(n) = current {
            if seen[n] { return Err(format!("cyclic parent graph at '{}'", parts[n].id)); }
            seen[n] = true;
            current = parts[n].parent;
        }
    }
    let mut clips = HashMap::new();
    if let Some(value) = map.get("motions") {
        for (id, value) in value.object()? {
            let anim = pack.animations.iter().find(|a| a.id == *id)
                .ok_or_else(|| format!("motion references unknown animation '{id}'"))?;
            clips.insert(id.clone(), clip_value(value, &ids, anim.frames)?);
        }
    }
    Ok(Rig { native, pack, parts, clips, locks })
}
fn interpolate(a: Pose, b: Pose, t: f64) -> Pose {
    // Shortest-arc interpolation prevents the long way around 180 degrees.
    let angle_delta = (b.angle - a.angle + 180.0).rem_euclid(360.0) - 180.0;
    Pose {
        dx: a.dx + (b.dx - a.dx) * t,
        dy: a.dy + (b.dy - a.dy) * t,
        angle: a.angle + angle_delta * t,
        scale: a.scale + (b.scale - a.scale) * t,
    }
}
fn poses(clip: Option<&Vec<Keyframe>>, frame: u32, parts: &[Part]) -> Vec<Pose> {
    let mut output = vec![Pose::default(); parts.len()];
    let Some(list) = clip else { return output; };
    let right = list.partition_point(|k| k.frame < frame);
    let (a, b, t) = if right == 0 { (&list[0], &list[0], 0.0) }
        else if right >= list.len() { (&list[list.len()-1], &list[list.len()-1], 0.0) }
        else {
            let prev = &list[right - 1]; let next = &list[right];
            (prev, next, (frame - prev.frame) as f64 / (next.frame - prev.frame) as f64)
        };
    for (i, part) in parts.iter().enumerate() {
        let before = a.poses.get(&part.id).copied().unwrap_or_default();
        let after = b.poses.get(&part.id).copied().unwrap_or_default();
        output[i] = interpolate(before, after, t);
    }
    output
}
fn matrix_for(i: usize, parts: &[Part], values: &[Pose], cache: &mut [Option<Mat>]) -> Mat {
    if let Some(x) = cache[i] { return x; }
    let part = &parts[i]; let value = values[i];
    let (s, c) = value.angle.to_radians().sin_cos();
    let a = c * value.scale; let b = s * value.scale;
    let cc = -s * value.scale; let d = c * value.scale;
    let (px, py) = part.pivot;
    let local = Mat {
        a, b, c: cc, d,
        tx: part.at.0 + value.dx - a * px - cc * py,
        ty: part.at.1 + value.dy - b * px - d * py,
    };
    let world = if let Some(parent) = part.parent {
        matrix_for(parent, parts, values, cache).mul(local)
    } else { local };
    cache[i] = Some(world);
    world
}
fn sample(image: &Image, x: f64, y: f64) -> [f64; 4] {
    let x0 = x.floor() as i32; let y0 = y.floor() as i32;
    let fx = x - x0 as f64; let fy = y - y0 as f64;
    let mut rgba = [0.0; 4]; // premultiplied RGB and alpha, normalized to [0,1]
    for dy in 0..=1 {
        for dx in 0..=1 {
            let ix = x0 + dx; let iy = y0 + dy;
            if ix < 0 || iy < 0 || ix >= image.width as i32 || iy >= image.height as i32 { continue; }
            let weight = if dx == 0 { 1.0 - fx } else { fx } *
                         if dy == 0 { 1.0 - fy } else { fy };
            let k = ((iy as u32 * image.width + ix as u32) * 4) as usize;
            let alpha = image.rgba[k + 3] as f64 / 255.0;
            rgba[3] += weight * alpha;
            for c in 0..3 { rgba[c] += weight * alpha * (image.rgba[k + c] as f64 / 255.0); }
        }
    }
    rgba
}
fn draw(dest: &mut Image, part: &Part, mat: Mat) {
    let corners = [
        mat.point(-0.5, -0.5), mat.point(part.image.width as f64 - 0.5, -0.5),
        mat.point(-0.5, part.image.height as f64 - 0.5),
        mat.point(part.image.width as f64 - 0.5, part.image.height as f64 - 0.5),
    ];
    let x0 = corners.iter().map(|v| v.0).fold(f64::INFINITY, f64::min).floor().max(0.0) as u32;
    let y0 = corners.iter().map(|v| v.1).fold(f64::INFINITY, f64::min).floor().max(0.0) as u32;
    let x1 = ((corners.iter().map(|v| v.0).fold(f64::NEG_INFINITY, f64::max) + 1.0).ceil()
        .max(0.0) as u32).min(dest.width);
    let y1 = ((corners.iter().map(|v| v.1).fold(f64::NEG_INFINITY, f64::max) + 1.0).ceil()
        .max(0.0) as u32).min(dest.height);
    let det = mat.a * mat.d - mat.b * mat.c;
    if det <= 0.0000001 { return; }
    for y in y0..y1 {
        for x in x0..x1 {
            let xx = x as f64 - mat.tx; let yy = y as f64 - mat.ty;
            let sx = (mat.d * xx - mat.c * yy) / det;
            let sy = (-mat.b * xx + mat.a * yy) / det;
            if sx < -1.0 || sy < -1.0 || sx > part.image.width as f64 ||
                sy > part.image.height as f64 { continue; }
            let src = sample(&part.image, sx, sy);
            if src[3] <= 0.0 { continue; }
            let offset = ((y * dest.width + x) * 4) as usize;
            let da = dest.rgba[offset + 3] as f64 / 255.0;
            let oa = src[3] + da * (1.0 - src[3]);
            if oa <= 0.0 { continue; }
            for channel in 0..3 {
                let d = dest.rgba[offset + channel] as f64 / 255.0 * da;
                let out = (src[channel] + d * (1.0 - src[3])) / oa;
                dest.rgba[offset + channel] = (out * 255.0).round().clamp(0.0, 255.0) as u8;
            }
            dest.rgba[offset + 3] = (oa * 255.0).round().clamp(0.0, 255.0) as u8;
        }
    }
}
fn render(rig: &Rig, animation: usize, frame: u32) -> Result<Image> {
    let a = &rig.pack.animations[animation];
    let values = poses(rig.clips.get(&a.id), frame, &rig.parts);
    let mut cache = vec![None; rig.parts.len()];
    let mut result = Image::blank(rig.pack.width, rig.pack.height)?;
    // Author-provided array order is compositing/z-order, not parent order.
    for i in 0..rig.parts.len() {
        let transform = matrix_for(i, &rig.parts, &values, &mut cache);
        draw(&mut result, &rig.parts[i], transform);
    }
    Ok(result)
}
pub fn build(source: &Path, output: &Path) -> Result<()> {
    let rig = read_rig(source)?;
    let width = rig.pack.animations.iter().map(|a| a.frames * rig.pack.width).max().unwrap();
    let height = rig.pack.animations.iter().map(|a| (a.row + 1) * rig.pack.height).max().unwrap();
    let mut atlas = Image::blank(width, height)?;
    let mut jobs = Vec::<(usize, u32)>::new();
    for (idx, a) in rig.pack.animations.iter().enumerate() {
        for frame in 0..a.frames { jobs.push((idx, frame)); }
    }
    // Render independent frames on CPU worker threads, with a single writer.
    // The atlas and diagnostics are produced in canonical animation/frame order,
    // regardless of scheduling.
    let index = AtomicUsize::new(0);
    let nthreads = thread::available_parallelism().map(|v| v.get()).unwrap_or(1)
        .min(8).min(jobs.len()).max(1);
    let mut occupied = HashMap::<(u32, u32), (usize, u32)>::new();
    thread::scope(|scope| -> Result<()> {
        let (sender, receiver) = mpsc::channel::<(usize, Result<Image>)>();
        for _ in 0..nthreads {
            let sender = sender.clone();
            let index = &index; let jobs = &jobs; let rig = &rig;
            scope.spawn(move || {
                loop {
                    let n = index.fetch_add(1, Ordering::Relaxed);
                    if n >= jobs.len() { break; }
                    let (animation, frame) = jobs[n];
                    if sender.send((n, render(rig, animation, frame))).is_err() { break; }
                }
            });
        }
        drop(sender);
        for _ in 0..jobs.len() {
            let (n, image) = receiver.recv().map_err(|_| "rig worker closed before finishing all frames")?;
            let (anim, frame) = jobs[n];
            let image = image?;
            let a = &rig.pack.animations[anim];
            let key = (a.row, frame);
            if let Some(&(prev_anim, prev_frame)) = occupied.get(&key) {
                let prior = atlas.cell(frame * rig.pack.width, a.row * rig.pack.height,
                    rig.pack.width, rig.pack.height)?;
                if prior != image {
                    return Err(format!("animations '{}' and '{}' disagree on atlas cell row {} column {} (frames {} and {})",
                        rig.pack.animations[prev_anim].id, a.id, a.row, frame, prev_frame, frame));
                }
            } else {
                atlas.paste(frame * rig.pack.width, a.row * rig.pack.height, &image)?;
                occupied.insert(key, (anim, frame));
            }
        }
        Ok(())
    })?;
    let mut audit = Audit::default();
    for a in &rig.pack.animations {
        let mut first: Option<Image> = None;
        let mut previous: Option<Image> = None;
        for frame in 0..a.frames {
            let image = atlas.cell(frame * rig.pack.width, a.row * rig.pack.height,
                rig.pack.width, rig.pack.height)?;
            measure(&image, &mut audit, &a.id, frame)?;
            if let Some(base) = &first {
                for rect in rig.locks.get(&a.id).into_iter().flatten() {
                    if !super::equal_region(base, &image, rect) {
                        return Err(format!("locked region changed in '{}' frame {}", a.id, frame));
                    }
                }
            } else { first = Some(image.clone()); }
            if previous.as_ref() == Some(&image) {
                warn(&mut audit, &a.id, frame, "identical-neighbor",
                    "Consecutive frames are identical (may be an intentional hold)");
            }
            previous = Some(image);
        }
    }
    let png = png::encode(&atlas)?;
    let manifest = format!("{}\n", json::stringify(&rig.native));
    if manifest.len() > 65536 { return Err("generated manifest exceeds 64 KiB".into()); }
    let report = obj(vec![
        ("schemaVersion", num(1)), ("generator", text("deskling-rig/1")),
        ("packId", text(&rig.pack.id)), ("checkedFrames", num(audit.frames as u32)),
        ("atlasWidth", num(atlas.width)), ("atlasHeight", num(atlas.height)),
        ("cpuWorkers", num(nthreads as u32)),
        ("warnings", Json::Array(audit.warnings)),
    ]);
    new_output(output)?;
    fs::write(output.join(&rig.pack.sheet), png).map_err(|e| e.to_string())?;
    fs::write(output.join("pack.json"), manifest).map_err(|e| e.to_string())?;
    fs::write(output.join("quality-report.json"),
        format!("{}\n", json::stringify(&report))).map_err(|e| e.to_string())?;
    println!("Rig built {}: {} layers, {} frame references, {} CPU workers, {}x{} atlas",
        rig.pack.id, rig.parts.len(), audit.frames, nthreads, width, height);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parent_matrix_propagates_motion() {
        let solid = Image::blank(16,16).unwrap();
        let parts = vec![
            Part {id:"body".into(),image:solid.clone(),parent:None,pivot:(0.,0.),at:(10.,10.)},
            Part {id:"arm".into(),image:solid,parent:Some(0),pivot:(0.,0.),at:(5.,0.)},
        ];
        let poses = [Pose{dx:2.,..Pose::default()},Pose::default()];
        let mut cache=vec![None;2];
        let p=matrix_for(1,&parts,&poses,&mut cache).point(0.,0.);
        assert_eq!(p,(17.,10.));
    }
    #[test]
    fn interpolation_is_short_arc() {
        let a=Pose{angle:170.,..Pose::default()};
        let b=Pose{angle:-170.,..Pose::default()};
        assert!((interpolate(a,b,0.5).angle-180.).abs()<0.0001);
    }
    #[test]
    fn bilinear_sampling_avoids_hidden_rgb_fringe() {
        let mut i=Image::blank(16,16).unwrap();
        i.rgba[0..4].copy_from_slice(&[255,0,0,0]); // hidden bright red must not leak
        i.rgba[4..8].copy_from_slice(&[0,128,255,255]);
        let s=sample(&i,0.5,0.0);
        assert_eq!(s[3],0.5);
        assert!(s[0].abs()<0.001);
    }
}
