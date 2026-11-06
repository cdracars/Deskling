// SPDX-License-Identifier: MIT
//! A reproducible synthetic multi-part mascot used to prove the rig pipeline.
//! All pixels are generated offline; it contains no Kipwick/private artwork.
use super::{Image, Result, new_output, obj, num, text};
use super::png;
use std::fs;
use std::path::Path;

fn blend_ellipse(image: &mut Image, cx: f64, cy: f64, rx: f64, ry: f64, color: [u8; 4]) {
    for y in 0..image.height {
        for x in 0..image.width {
            let nx = (x as f64 - cx) / rx;
            let ny = (y as f64 - cy) / ry;
            if nx * nx + ny * ny <= 1.0 {
                let i = ((y * image.width + x) * 4) as usize;
                image.rgba[i..i + 4].copy_from_slice(&color);
            }
        }
    }
}
fn line(image: &mut Image, a: (i32, i32), b: (i32, i32), radius: i32, color: [u8; 4]) {
    let vx = (b.0 - a.0) as f64;
    let vy = (b.1 - a.1) as f64;
    let denom = (vx * vx + vy * vy).max(1.0);
    for y in 0..image.height as i32 {
        for x in 0..image.width as i32 {
            let wx = x as f64 - a.0 as f64;
            let wy = y as f64 - a.1 as f64;
            let t = ((wx * vx + wy * vy) / denom).clamp(0.0, 1.0);
            let dx = wx - t * vx;
            let dy = wy - t * vy;
            if dx * dx + dy * dy <= (radius * radius) as f64 {
                let i = ((y as u32 * image.width + x as u32) * 4) as usize;
                image.rgba[i..i + 4].copy_from_slice(&color);
            }
        }
    }
}
fn part(dir: &Path, name: &str, image: Image) -> Result<()> {
    fs::write(dir.join(name), png::encode(&image)?).map_err(|e| e.to_string())
}
fn save(dir: &Path) -> Result<()> {
    let mut body = Image::blank(96, 90)?;
    blend_ellipse(&mut body, 47.0, 45.0, 42.0, 42.0, [65, 58, 116, 255]);
    blend_ellipse(&mut body, 46.0, 57.0, 25.0, 21.0, [111, 97, 168, 255]);
    blend_ellipse(&mut body, 30.0, 31.0, 13.0, 7.0, [90, 78, 146, 255]);
    part(dir, "body.png", body)?;
    let mut left = Image::blank(32, 60)?;
    line(&mut left, (16, 10), (13, 43), 10, [52, 48, 108, 255]);
    blend_ellipse(&mut left, 13.0, 45.0, 13.0, 12.0, [78, 76, 151, 255]);
    part(dir, "left-arm.png", left)?;
    let mut right = Image::blank(32, 60)?;
    line(&mut right, (16, 10), (18, 43), 10, [52, 48, 108, 255]);
    blend_ellipse(&mut right, 18.0, 45.0, 13.0, 12.0, [78, 76, 151, 255]);
    part(dir, "right-arm.png", right)?;
    let mut foot = Image::blank(34, 20)?;
    blend_ellipse(&mut foot, 17.0, 11.0, 16.0, 8.0, [43, 37, 83, 255]);
    part(dir, "foot.png", foot)?;
    let mut eye = Image::blank(16, 16)?;
    blend_ellipse(&mut eye, 8.0, 8.0, 7.0, 7.0, [110, 242, 241, 255]);
    blend_ellipse(&mut eye, 10.0, 9.0, 2.0, 4.0, [16, 35, 64, 255]);
    part(dir, "eye.png", eye)?;
    let mut smile = Image::blank(40, 20)?;
    // A small smile formed from one connected dark arc; an invariant image layer.
    for x in 0..40i32 {
        let t = (x as f64 - 19.5) / 19.5;
        let y = (7.0 + 8.0 * (1.0 - t * t)).round() as i32;
        line(&mut smile, (x, y), (x, y), 2, [22, 24, 54, 255]);
    }
    part(dir, "smile.png", smile)?;
    let mut antenna = Image::blank(26, 42)?;
    line(&mut antenna, (13, 38), (13, 12), 3, [34, 32, 74, 255]);
    blend_ellipse(&mut antenna, 13.0, 9.0, 8.0, 8.0, [125, 224, 221, 255]);
    part(dir, "antenna.png", antenna)?;
    Ok(())
}
pub fn create(folder: &Path) -> Result<()> {
    new_output(folder)?;
    let layers = folder.join("layers");
    fs::create_dir_all(&layers).map_err(|e| e.to_string())?;
    save(&layers)?;
    // Shared native manifest. Authoring-only joints/motions are outside pack.
    let json = r##"{
      "schemaVersion": 1,
      "pack": {
        "schemaVersion": 1, "id": "rust-rig-demo", "name": "Rust Rig Demo",
        "spriteSheet": "sprites.png", "cellWidth": 192, "cellHeight": 192,
        "idle": "idle",
        "animations": [
          {"id": "idle", "label": "Breathing", "row": 0, "frames": 16, "fps": 10},
          {"id": "blink", "label": "Blink", "row": 1, "frames": 8, "fps": 16},
          {"id": "wave", "label": "Wave", "row": 2, "frames": 16, "fps": 16},
          {"id": "bounce", "label": "Bounce", "row": 3, "frames": 16, "fps": 16}
        ],
        "interactions": {"wave":"wave"},
        "behaviors": [
          {"animation":"blink","loops":1,"weight":8},
          {"animation":"bounce","loops":1,"weight":1}
        ]
      },
      "parts": [
        {"id":"foot-left","image":"foot.png","parent":"body","pivot":[17,4],"at":[30,85]},
        {"id":"foot-right","image":"foot.png","parent":"body","pivot":[17,4],"at":[66,85]},
        {"id":"arm-left","image":"left-arm.png","parent":"body","pivot":[16,10],"at":[12,48]},
        {"id":"arm-right","image":"right-arm.png","parent":"body","pivot":[16,10],"at":[84,48]},
        {"id":"antenna","image":"antenna.png","parent":"body","pivot":[13,38],"at":[48,10]},
        {"id":"body","image":"body.png","pivot":[48,45],"at":[96,103]},
        {"id":"eye-left","image":"eye.png","parent":"body","pivot":[8,8],"at":[31,33]},
        {"id":"eye-right","image":"eye.png","parent":"body","pivot":[8,8],"at":[65,33]},
        {"id":"mouth","image":"smile.png","parent":"body","pivot":[20,10],"at":[48,58]}
      ],
      "motions": {
        "idle": [
          {"frame":0,"poses":{"body":{"scaleY":1.0}}},
          {"frame":7,"poses":{"body":{"scaleY":1.035}}},
          {"frame":15,"poses":{"body":{"scaleY":1.0}}}
        ],
        "blink": [
          {"frame":0,"poses":{"eye-left":{"scaleY":1},"eye-right":{"scaleY":1}}},
          {"frame":3,"poses":{"eye-left":{"scaleY":0.15},"eye-right":{"scaleY":0.15}}},
          {"frame":7,"poses":{"eye-left":{"scaleY":1},"eye-right":{"scaleY":1}}}
        ],
        "wave": [
          {"frame":0,"poses":{"arm-right":{"rotate":0}}},
          {"frame":4,"poses":{"arm-right":{"rotate":-105}}},
          {"frame":8,"poses":{"arm-right":{"rotate":-75}}},
          {"frame":11,"poses":{"arm-right":{"rotate":-110}}},
          {"frame":15,"poses":{"arm-right":{"rotate":0}}}
        ],
        "bounce": [
          {"frame":0,"poses":{"body":{"dy":0}}},
          {"frame":3,"poses":{"body":{"dy":8,"scaleY":0.94}}},
          {"frame":7,"poses":{"body":{"dy":-16,"scaleY":1.06}}},
          {"frame":12,"poses":{"body":{"dy":3}}},
          {"frame":15,"poses":{"body":{"dy":0}}}
        ]
      },
      "lockedRegions": {
        "wave":[{"x":76,"y":76,"width":40,"height":20}]
      }
    }"##;
    // Parse this fixture during creation, so shipping a broken example fails.
    super::json::parse(json.as_bytes())?;
    fs::write(folder.join("rig.json"), format!("{json}\n")).map_err(|e| e.to_string())?;
    println!("Wrote independent layered character example to {}", folder.display());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rig;
    use std::env;
    #[test]
    fn demo_rig_build_is_deterministic_and_has_motion() {
        let root = env::temp_dir().join(format!("deskling-rig-demo-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let source = root.join("source");
        create(&source).unwrap();
        rig::build(&source.join("rig.json"), &root.join("out-a")).unwrap();
        rig::build(&source.join("rig.json"), &root.join("out-b")).unwrap();
        let a = fs::read(root.join("out-a/sprites.png")).unwrap();
        let b = fs::read(root.join("out-b/sprites.png")).unwrap();
        assert_eq!(a, b, "CPU worker scheduling cannot change output");
        let atlas = png::decode(&a).unwrap();
        assert_eq!((atlas.width,atlas.height),(192*16,192*4));
        let left = atlas.cell(0,192*2,192,192).unwrap();
        let wave = atlas.cell(192*6,192*2,192,192).unwrap();
        assert_ne!(left.rgba, wave.rgba, "wave must have real intermediate pixels");
        let blink = atlas.cell(192*3,192,192,192).unwrap();
        let open = atlas.cell(0,192,192,192).unwrap();
        assert_ne!(blink.rgba, open.rgba, "blink must change eye geometry");
        fs::remove_dir_all(root).unwrap();
    }
}
