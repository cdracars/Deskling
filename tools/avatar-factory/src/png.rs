// SPDX-License-Identifier: MIT
//! Bounded, deterministic RGBA8 PNG codec using system zlib (libz).
//! This is a Linux authoring utility, not an image-generation model.
use std::os::raw::{c_int, c_ulong};

#[link(name = "z")]
unsafe extern "C" {
    fn uncompress(dest: *mut u8, dest_len: *mut c_ulong, src: *const u8, src_len: c_ulong) -> c_int;
    fn compress2(dest: *mut u8, dest_len: *mut c_ulong, src: *const u8, src_len: c_ulong, level: c_int) -> c_int;
    fn compressBound(source_len: c_ulong) -> c_ulong;
}
pub const MAX_PIXELS: usize = 16_777_216;
pub const MAX_PNG: usize = 16 * 1024 * 1024;
const MAGIC: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image { pub width: u32, pub height: u32, pub rgba: Vec<u8> }
impl Image {
    pub fn blank(width: u32, height: u32) -> Result<Self, String> {
        let size = checked_size(width, height)?;
        Ok(Self { width, height, rgba: vec![0; size] })
    }
    pub fn cell(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Self, String> {
        let xend = x.checked_add(width).ok_or("cell x overflow")?;
        let yend = y.checked_add(height).ok_or("cell y overflow")?;
        if xend > self.width || yend > self.height { return Err("cell outside PNG".into()); }
        let mut result = Self::blank(width, height)?;
        for row in 0..height as usize {
            let src = ((y as usize + row) * self.width as usize + x as usize) * 4;
            let dst = row * width as usize * 4;
            result.rgba[dst..dst + width as usize * 4].copy_from_slice(&self.rgba[src..src + width as usize * 4]);
        }
        Ok(result)
    }
    pub fn paste(&mut self, x: u32, y: u32, image: &Image) -> Result<(), String> {
        if x.checked_add(image.width).ok_or("x overflow")? > self.width ||
            y.checked_add(image.height).ok_or("y overflow")? > self.height { return Err("frame outside atlas".into()); }
        for row in 0..image.height as usize {
            let dst = ((y as usize + row) * self.width as usize + x as usize) * 4;
            let src = row * image.width as usize * 4;
            self.rgba[dst..dst + image.width as usize * 4].copy_from_slice(&image.rgba[src..src + image.width as usize * 4]);
        }
        Ok(())
    }
}
fn checked_size(width: u32, height: u32) -> Result<usize, String> {
    if width == 0 || height == 0 || width > 16384 || height > 16384 ||
        (width as u64) * (height as u64) > MAX_PIXELS as u64 {
        return Err("PNG exceeds 16384-side / 16777216-pixel limits".into());
    }
    Ok(width as usize * height as usize * 4)
}
fn be(bytes: &[u8]) -> u32 { u32::from_be_bytes(bytes.try_into().unwrap()) }
fn crc(bytes: &[u8]) -> u32 {
    let mut value = 0xffff_ffffu32;
    for b in bytes {
        value ^= *b as u32;
        for _ in 0..8 { value = (value >> 1) ^ (if value & 1 != 0 { 0xedb8_8320 } else { 0 }); }
    }
    !value
}
pub fn decode(data: &[u8]) -> Result<Image, String> {
    if data.len() > MAX_PNG { return Err("PNG file exceeds 16 MiB".into()); }
    if !data.starts_with(MAGIC) { return Err("invalid PNG signature".into()); }
    let mut pos = 8usize;
    let mut dimensions = None;
    let mut compressed = Vec::<u8>::new();
    let mut ended = false;
    let mut saw_idat = false;
    while pos < data.len() {
        if data.len() - pos < 12 { return Err("truncated PNG chunk".into()); }
        let len = be(&data[pos..pos + 4]) as usize;
        let end = pos.checked_add(12).and_then(|v| v.checked_add(len)).ok_or("PNG chunk size overflow")?;
        if end > data.len() { return Err("truncated PNG chunk".into()); }
        let kind = &data[pos + 4..pos + 8];
        let payload = &data[pos + 8..pos + 8 + len];
        if crc(&data[pos + 4..pos + 8 + len]) != be(&data[pos + 8 + len..end]) {
            return Err("PNG chunk CRC mismatch".into());
        }
        match kind {
            b"IHDR" => {
                if dimensions.is_some() || pos != 8 || len != 13 { return Err("invalid PNG IHDR".into()); }
                let width = be(&payload[0..4]); let height = be(&payload[4..8]);
                checked_size(width, height)?;
                if payload[8..] != [8, 6, 0, 0, 0] {
                    return Err("only non-interlaced 8-bit RGBA PNGs are supported for authoring".into());
                }
                dimensions = Some((width, height));
            }
            b"IDAT" => {
                if dimensions.is_none() { return Err("IDAT before IHDR".into()); }
                saw_idat = true;
                if compressed.len() + len > MAX_PNG { return Err("compressed PNG too large".into()); }
                compressed.extend_from_slice(payload);
            }
            b"IEND" => {
                if len != 0 || !saw_idat { return Err("invalid PNG end".into()); }
                if end != data.len() { return Err("extra bytes after IEND".into()); }
                ended = true;
                break;
            }
            _ => {
                if kind[0] & 32 == 0 { return Err("unsupported critical PNG chunk".into()); }
            }
        }
        pos = end;
    }
    if !ended { return Err("PNG has no IEND".into()); }
    let (width, height) = dimensions.ok_or("missing IHDR")?;
    let stride = width as usize * 4;
    let row_len = stride + 1;
    let mut raw = vec![0; row_len * height as usize];
    let mut raw_len = raw.len() as c_ulong;
    let status = unsafe { uncompress(raw.as_mut_ptr(), &mut raw_len, compressed.as_ptr(), compressed.len() as c_ulong) };
    if status != 0 || raw_len as usize != raw.len() { return Err("PNG inflate failed or decompressed length mismatch".into()); }
    let mut image = Image::blank(width, height)?;
    for row in 0..height as usize {
        let filter = raw[row * row_len];
        if filter > 4 { return Err("unsupported PNG row filter".into()); }
        for col in 0..stride {
            let a = if col >= 4 { image.rgba[row * stride + col - 4] } else { 0 };
            let b = if row > 0 { image.rgba[(row - 1) * stride + col] } else { 0 };
            let c = if row > 0 && col >= 4 { image.rgba[(row - 1) * stride + col - 4] } else { 0 };
            let predictor = match filter {
                0 => 0, 1 => a, 2 => b, 3 => ((a as u16 + b as u16) / 2) as u8,
                _ => {
                    let p = a as i32 + b as i32 - c as i32;
                    let da = (p - a as i32).abs(); let db = (p - b as i32).abs(); let dc = (p - c as i32).abs();
                    if da <= db && da <= dc { a } else if db <= dc { b } else { c }
                }
            };
            image.rgba[row * stride + col] = raw[row * row_len + 1 + col].wrapping_add(predictor);
        }
    }
    Ok(image)
}
fn chunk(output: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    output.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let start = output.len();
    output.extend_from_slice(kind);
    output.extend_from_slice(data);
    output.extend_from_slice(&crc(&output[start..]).to_be_bytes());
}
pub fn encode(image: &Image) -> Result<Vec<u8>, String> {
    if image.rgba.len() != checked_size(image.width, image.height)? { return Err("RGBA buffer size mismatch".into()); }
    let stride = image.width as usize * 4;
    let mut raw = Vec::with_capacity((stride + 1) * image.height as usize);
    for row in 0..image.height as usize {
        raw.push(0); // deterministic filter 0
        raw.extend_from_slice(&image.rgba[row * stride..(row + 1) * stride]);
    }
    let bound = unsafe { compressBound(raw.len() as c_ulong) } as usize;
    let mut compressed = vec![0u8; bound];
    let mut n = bound as c_ulong;
    let status = unsafe { compress2(compressed.as_mut_ptr(), &mut n, raw.as_ptr(), raw.len() as c_ulong, 6) };
    if status != 0 { return Err(format!("zlib compression failed: {status}")); }
    compressed.truncate(n as usize);
    let mut output = MAGIC.to_vec();
    let mut header = Vec::with_capacity(13);
    header.extend_from_slice(&image.width.to_be_bytes());
    header.extend_from_slice(&image.height.to_be_bytes());
    header.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(&mut output, b"IHDR", &header);
    chunk(&mut output, b"IDAT", &compressed);
    chunk(&mut output, b"IEND", &[]);
    if output.len() > MAX_PNG { return Err("encoded PNG exceeds 16 MiB".into()); }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn exact_roundtrip_and_chunk_crc() {
        let mut image = Image::blank(16, 16).unwrap();
        for i in 0..image.rgba.len() { image.rgba[i] = ((i * 37) % 256) as u8; }
        let png = encode(&image).unwrap();
        assert_eq!(decode(&png).unwrap(), image);
        let mut corrupted = png.clone(); corrupted[30] ^= 1;
        assert!(decode(&corrupted).is_err());
    }
    #[test] fn cells_and_bounds() {
        let mut image = Image::blank(32, 16).unwrap();
        let mut cell = Image::blank(16, 16).unwrap();
        cell.rgba[3] = 255;
        image.paste(16, 0, &cell).unwrap();
        assert_eq!(image.cell(16, 0, 16, 16).unwrap(), cell);
        assert!(image.cell(20, 0, 16, 16).is_err());
    }
}
