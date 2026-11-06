// SPDX-License-Identifier: MIT
//! Deskling Avatar Factory: inspect native packs, extract editable frame projects,
//! and rebuild a deterministic schema-1 runtime atlas on Linux.
mod json;
mod png;
mod rig;
use json::{field, only, Json};
use png::Image;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, String>;
#[derive(Clone, Debug)]
struct Animation { id: String, label: String, row: u32, frames: u32, fps: u32 }
#[derive(Clone, Debug)]
struct Pack { id: String, name: String, sheet: String, width: u32, height: u32, animations: Vec<Animation> }
#[derive(Clone, Debug)]
struct Region { x: u32, y: u32, w: u32, h: u32 }
#[derive(Default)]
struct Audit {
    warnings: Vec<Json>,
    frames: usize,
}
fn obj(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}
fn text(s: impl Into<String>) -> Json { Json::Text(s.into()) }
fn num(n: u32) -> Json { Json::Number(n as f64) }
fn get_text(map: &BTreeMap<String, Json>, key: &str, max: usize) -> Result<String> {
    let s = field(map, key)?.string()?;
    if s.trim().is_empty() || s.chars().count() > max || s.chars().any(|c| c.is_control()) || (s.contains('<') || s.contains('>')) {
        return Err(format!("'{key}' must be nonempty plain text (max {max} characters)"));
    }
    Ok(s.to_string())
}
fn get_int(map: &BTreeMap<String, Json>, key: &str, a: u32, b: u32) -> Result<u32> {
    field(map, key)?.integer(a, b).map_err(|e| format!("'{key}': {e}"))
}
fn slug(s: &str) -> bool {
    s.len() <= 64 && !s.is_empty() && s.as_bytes()[0].is_ascii_lowercase() &&
        !s.ends_with('-') && !s.contains("--") &&
        s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
fn parse_pack(root: &Json) -> Result<Pack> {
    let m = root.object()?;
    only(m, &["schemaVersion", "id", "name", "spriteSheet", "cellWidth", "cellHeight", "animations",
        "idle", "interactions", "behaviors", "metadata", "author", "license"])?;
    if get_int(m, "schemaVersion", 1, 1)? != 1 { return Err("unsupported schema".into()); }
    let id = get_text(m, "id", 64)?;
    if !slug(&id) { return Err("pack id must be a lowercase slug".into()); }
    let name = get_text(m, "name", 128)?;
    let sheet = get_text(m, "spriteSheet", 255)?;
    if !sheet.to_ascii_lowercase().ends_with(".png") || sheet.eq_ignore_ascii_case(".png") ||
        (sheet.contains('/') || sheet.contains('\\') || sheet.contains(':')) || sheet == "." || sheet == ".." {
        return Err("spriteSheet must be a local PNG basename".into());
    }
    let width = get_int(m, "cellWidth", 16, 512)?;
    let height = get_int(m, "cellHeight", 16, 512)?;
    let idle = get_text(m, "idle", 64)?;
    for key in ["author", "license"] { if m.contains_key(key) { get_text(m, key, 512)?; } }
    if let Some(metadata) = m.get("metadata") {
        let mm = metadata.object()?;
        only(mm, &["author", "license"])?;
        for key in mm.keys() { get_text(mm, key, 512)?; }
    }
    let entries = field(m, "animations")?.array()?;
    if entries.is_empty() || entries.len() > 32 { return Err("animations must have 1 to 32 entries".into()); }
    let mut animations = Vec::new();
    for entry in entries {
        let a = entry.object()?;
        only(a, &["id", "label", "row", "frames", "fps"])?;
        let aid = get_text(a, "id", 64)?;
        if !slug(&aid) || animations.iter().any(|v: &Animation| v.id == aid) {
            return Err(format!("duplicate or invalid animation id: {aid}"));
        }
        animations.push(Animation {
            id: aid, label: get_text(a, "label", 128)?,
            row: get_int(a, "row", 0, 255)?, frames: get_int(a, "frames", 1, 64)?,
            fps: get_int(a, "fps", 1, 60)?,
        });
    }
    if !animations.iter().any(|a| a.id == idle) { return Err("idle references an unknown animation".into()); }
    if let Some(interactions) = m.get("interactions") {
        let map = interactions.object()?;
        only(map, &["wave", "dragLeft", "dragRight"])?;
        for (key, value) in map {
            if !animations.iter().any(|a| a.id == value.string().unwrap_or("")) {
                return Err(format!("interaction '{key}' references an unknown animation"));
            }
        }
    }
    if let Some(behaviors) = m.get("behaviors") {
        let list = behaviors.array()?;
        if list.len() > 32 { return Err("at most 32 behaviors".into()); }
        for v in list {
            let b = v.object()?; only(b, &["animation", "loops", "weight"])?;
            let aid = get_text(b, "animation", 64)?;
            if !animations.iter().any(|a| a.id == aid) { return Err("unknown behavior animation".into()); }
            get_int(b, "loops", 1, 10)?;
            get_int(b, "weight", 1, 100)?;
        }
    }
    Ok(Pack { id, name, sheet, width, height, animations })
}
fn parse_project(root: &Json) -> Result<(Json, Pack, BTreeMap<String, Vec<Region>>)> {
    let m = root.object()?;
    only(m, &["schemaVersion", "pack", "lockedRegions"])?;
    get_int(m, "schemaVersion", 1, 1)?;
    let pack_json = field(m, "pack")?.clone();
    let pack = parse_pack(&pack_json)?;
    let mut regions = BTreeMap::new();
    if let Some(locks) = m.get("lockedRegions") {
        for (name, entries) in locks.object()? {
            if !pack.animations.iter().any(|a| &a.id == name) { return Err(format!("unknown locked animation '{name}'")); }
            let mut parsed = Vec::new();
            for region in entries.array()? {
                let r = region.object()?;
                only(r, &["x", "y", "width", "height"])?;
                let x = get_int(r, "x", 0, pack.width - 1)?;
                let y = get_int(r, "y", 0, pack.height - 1)?;
                let w = get_int(r, "width", 1, pack.width)?;
                let h = get_int(r, "height", 1, pack.height)?;
                if x + w > pack.width || y + h > pack.height { return Err("locked region outside cell".into()); }
                parsed.push(Region { x, y, w, h });
            }
            regions.insert(name.to_string(), parsed);
        }
    }
    Ok((pack_json, pack, regions))
}
fn read_json(path: &Path) -> Result<Json> {
    let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    json::parse(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}
fn sheet_path(manifest: &Path, name: &str) -> Result<PathBuf> {
    let parent = manifest.canonicalize().map_err(|e| e.to_string())?
        .parent().ok_or("manifest has no parent")?.to_path_buf();
    let path = parent.join(name).canonicalize().map_err(|e| format!("sprite sheet: {e}"))?;
    if path.parent() != Some(parent.as_path()) { return Err("spriteSheet resolves outside manifest directory".into()); }
    Ok(path)
}
fn verify_layout(pack: &Pack, image: &Image) -> Result<()> {
    for a in &pack.animations {
        if a.frames * pack.width > image.width || (a.row + 1) * pack.height > image.height {
            return Err(format!("animation '{}' extends outside atlas", a.id));
        }
    }
    Ok(())
}
fn new_output(path: &Path) -> Result<()> {
    if path.exists() {
        if !path.is_dir() { return Err("output is not a directory".into()); }
        if fs::read_dir(path).map_err(|e| e.to_string())?.next().is_some() {
            return Err("output directory must be empty; refusing to overwrite files".into());
        }
    } else {
        fs::create_dir_all(path).map_err(|e| format!("creating {}: {e}", path.display()))?;
    }
    Ok(())
}
fn frame_path(project_dir: &Path, name: &str, frame: u32) -> PathBuf {
    project_dir.join("frames").join(name).join(format!("{frame:04}.png"))
}
fn warn(audit: &mut Audit, animation: &str, index: u32, code: &str, detail: &str) {
    audit.warnings.push(obj(vec![
        ("animation", text(animation)), ("frame", num(index)),
        ("code", text(code)), ("detail", text(detail)),
    ]));
}
fn measure(image: &Image, audit: &mut Audit, animation: &str, frame: u32) -> Result<()> {
    let mut visible = 0u32;
    let mut transparent = 0u32;
    let mut edge = 0u32;
    for y in 0..image.height {
        for x in 0..image.width {
            let alpha = image.rgba[((y * image.width + x) * 4 + 3) as usize];
            if alpha >= 128 { visible += 1; }
            if alpha == 0 { transparent += 1; }
            if alpha != 0 && (x == 0 || x + 1 == image.width || y == 0 || y + 1 == image.height) {
                edge += 1;
            }
        }
    }
    if visible == 0 { return Err(format!("{animation}[{frame:04}] has no visible pixels")); }
    if transparent == 0 { warn(audit, animation, frame, "no-transparent-padding", "Character has no fully transparent pixels"); }
    if edge > 0 { warn(audit, animation, frame, "edge-touch", "Character touches an outer cell border"); }
    audit.frames += 1;
    Ok(())
}
fn equal_region(a: &Image, b: &Image, rect: &Region) -> bool {
    for y in rect.y..rect.y + rect.h {
        let start = ((y * a.width + rect.x) * 4) as usize;
        let end = start + rect.w as usize * 4;
        if a.rgba[start..end] != b.rgba[start..end] { return false; }
    }
    true
}
fn inspect(manifest: &Path) -> Result<()> {
    let root = read_json(manifest)?;
    let pack = parse_pack(&root)?;
    let sprite_path = sheet_path(manifest, &pack.sheet)?;
    let png = fs::read(&sprite_path).map_err(|e| e.to_string())?;
    let atlas = png::decode(&png)?;
    verify_layout(&pack, &atlas)?;
    let mut audit = Audit::default();
    println!("Pack {} ({})", pack.id, pack.name);
    println!("Atlas {}: {}x{}, {} bytes", pack.sheet, atlas.width, atlas.height, png.len());
    for a in &pack.animations {
        println!("  {:<16} row {:>3}, {:>2} frames, {:>2} fps", a.id, a.row, a.frames, a.fps);
        for index in 0..a.frames {
            let image = atlas.cell(index * pack.width, a.row * pack.height, pack.width, pack.height)?;
            measure(&image, &mut audit, &a.id, index)?;
        }
    }
    println!("Checked {} frame references, {} warnings", audit.frames, audit.warnings.len());
    for warning in &audit.warnings { println!("  {}", json::stringify(warning)); }
    Ok(())
}
fn extract(manifest: &Path, output: &Path) -> Result<()> {
    let root = read_json(manifest)?;
    let pack = parse_pack(&root)?;
    let png = fs::read(sheet_path(manifest, &pack.sheet)?).map_err(|e| e.to_string())?;
    let atlas = png::decode(&png)?;
    verify_layout(&pack, &atlas)?;
    // Validate before modifying the destination.
    for a in &pack.animations {
        for i in 0..a.frames {
            let frame = atlas.cell(i * pack.width, a.row * pack.height, pack.width, pack.height)?;
            let mut temp_audit = Audit::default();
            measure(&frame, &mut temp_audit, &a.id, i)?;
        }
    }
    new_output(output)?;
    for a in &pack.animations {
        fs::create_dir_all(output.join("frames").join(&a.id)).map_err(|e| e.to_string())?;
        for i in 0..a.frames {
            let frame = atlas.cell(i * pack.width, a.row * pack.height, pack.width, pack.height)?;
            fs::write(frame_path(output, &a.id, i), png::encode(&frame)?).map_err(|e| e.to_string())?;
        }
    }
    let project = obj(vec![
        ("schemaVersion", num(1)), ("pack", root),
        ("lockedRegions", Json::Object(BTreeMap::new())),
    ]);
    fs::write(output.join("project.json"), format!("{}\n", json::stringify(&project))).map_err(|e| e.to_string())?;
    println!("Extracted {} animation rows to {}", pack.animations.len(), output.display());
    Ok(())
}
fn build(project_file: &Path, output: &Path) -> Result<()> {
    let root = read_json(project_file)?;
    let (pack_json, pack, locks) = parse_project(&root)?;
    let project_dir = project_file.parent().unwrap_or_else(|| Path::new("."));
    let width = pack.animations.iter().map(|a| a.frames * pack.width).max().unwrap();
    let height = pack.animations.iter().map(|a| (a.row + 1) * pack.height).max().unwrap();
    let mut atlas = Image::blank(width, height)?;
    let mut occupied = HashMap::<(u32, u32), Vec<u8>>::new();
    let mut audit = Audit::default();
    for a in &pack.animations {
        let mut first: Option<Image> = None;
        let mut previous: Option<Image> = None;
        for i in 0..a.frames {
            let path = frame_path(project_dir, &a.id, i);
            let source = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let image = png::decode(&source).map_err(|e| format!("{}: {e}", path.display()))?;
            if image.width != pack.width || image.height != pack.height {
                return Err(format!("{}: expected {}x{} RGBA frame", path.display(), pack.width, pack.height));
            }
            measure(&image, &mut audit, &a.id, i)?;
            if let Some(start) = &first {
                for rect in locks.get(&a.id).into_iter().flatten() {
                    if !equal_region(start, &image, rect) {
                        return Err(format!("locked region changed in {} frame {i}", a.id));
                    }
                }
            } else { first = Some(image.clone()); }
            if previous.as_ref() == Some(&image) && i > 0 {
                warn(&mut audit, &a.id, i, "identical-neighbor", "Consecutive frames are identical (may be an intentional hold)");
            }
            previous = Some(image.clone());
            if let Some(pixels) = occupied.get(&(a.row, i)) {
                if pixels != &image.rgba {
                    return Err(format!("different animations disagree on shared atlas cell row={} column={i}", a.row));
                }
            } else {
                atlas.paste(i * pack.width, a.row * pack.height, &image)?;
                occupied.insert((a.row, i), image.rgba);
            }
        }
    }
    let bytes = png::encode(&atlas)?;
    let manifest = format!("{}\n", json::stringify(&pack_json));
    if manifest.len() > 65536 { return Err("generated manifest exceeds 64 KiB".into()); }
    let report = obj(vec![
        ("schemaVersion", num(1)), ("packId", text(&pack.id)), ("checkedFrames", num(audit.frames as u32)),
        ("atlasWidth", num(width)), ("atlasHeight", num(height)), ("warnings", Json::Array(audit.warnings)),
    ]);
    // The whole build is validated before creating the output directory.
    new_output(output)?;
    fs::write(output.join(&pack.sheet), bytes).map_err(|e| e.to_string())?;
    fs::write(output.join("pack.json"), manifest).map_err(|e| e.to_string())?;
    fs::write(output.join("quality-report.json"), format!("{}\n", json::stringify(&report))).map_err(|e| e.to_string())?;
    println!("Built {}: {} frames, {}x{} atlas", pack.id, audit.frames, width, height);
    Ok(())
}
fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, command, path] if command == "inspect" => inspect(Path::new(path)),
        [_, command, input, output] if command == "extract" => extract(Path::new(input), Path::new(output)),
        [_, command, input, output] if command == "build" => build(Path::new(input), Path::new(output)),
        [_, command, input, output] if command == "rig" => rig::build(Path::new(input), Path::new(output)),
        _ => Err("Usage:\n  deskling-avatar-factory inspect <pack.json>\n  deskling-avatar-factory extract <pack.json> <new-empty-project-dir>\n  deskling-avatar-factory build <project.json> <new-empty-output-dir>\n  deskling-avatar-factory rig <rig.json> <new-empty-output-dir>".into()),
    }
}
fn main() {
    if let Err(err) = run() { eprintln!("Avatar Factory: {err}"); std::process::exit(1); }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn slug_validation() {
        for bad in ["", "Bad", "x--y", "-foo", "foo-", "../abc", "x/y"] { assert!(!slug(bad)); }
        for good in ["kipwick", "cyber-gremlin", "c2"] { assert!(slug(good)); }
    }
    #[test]
    fn parses_existing_manifests() {
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/characters");
        for name in ["kipwick-alpha", "cyber-gremlin"] {
            let manifest = read_json(&base.join(name).join("pack.json")).unwrap();
            let pack = parse_pack(&manifest).unwrap();
            assert!(!pack.animations.is_empty());
            assert!(pack.width >= 16);
        }
    }
    #[test]
    fn build_extract_roundtrip_small_fixture() {
        let dir = env::temp_dir().join(format!("deskling-factory-{}-{}", std::process::id(),
            std::thread::current().name().unwrap_or("test")));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let spec = r#"{"schemaVersion":1,"pack":{"schemaVersion":1,"id":"fixture","name":"Fixture",
        "spriteSheet":"sprites.png","cellWidth":16,"cellHeight":16,"idle":"idle",
        "animations":[{"id":"idle","label":"Idle","row":0,"frames":2,"fps":8},
        {"id":"wave","label":"Wave","row":1,"frames":2,"fps":8}],
        "interactions":{"wave":"wave"}}}"#;
        fs::write(dir.join("project.json"), spec).unwrap();
        for (name, row) in [("idle", 0u8), ("wave", 1)] {
            fs::create_dir_all(dir.join("frames").join(name)).unwrap();
            for frame in 0..2u32 {
                let mut img = Image::blank(16, 16).unwrap();
                let p = ((row as usize + frame as usize) * 16 + 1) * 4;
                img.rgba[p] = 150; img.rgba[p + 3] = 255;
                fs::write(frame_path(&dir, name, frame), png::encode(&img).unwrap()).unwrap();
            }
        }
        build(&dir.join("project.json"), &dir.join("export")).unwrap();
        inspect(&dir.join("export/pack.json")).unwrap();
        extract(&dir.join("export/pack.json"), &dir.join("unpacked")).unwrap();
        build(&dir.join("unpacked/project.json"), &dir.join("rebuilt")).unwrap();
        let first = fs::read(dir.join("export/sprites.png")).unwrap();
        assert_eq!(first, fs::read(dir.join("rebuilt/sprites.png")).unwrap());
        assert_eq!(fs::read(dir.join("export/pack.json")).unwrap(),
                   fs::read(dir.join("rebuilt/pack.json")).unwrap());
        fs::remove_dir_all(dir).unwrap();
    }
}
