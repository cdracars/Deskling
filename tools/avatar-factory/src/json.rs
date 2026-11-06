// SPDX-License-Identifier: MIT
//! Small strict JSON reader for the data-only Deskling schema.
//! No network, scripts, or third-party dependencies.
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Json {
    Null,
    Bool,
    Number(f64),
    Text(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}
impl Json {
    pub fn object(&self) -> Result<&BTreeMap<String, Json>, String> {
        match self { Self::Object(v) => Ok(v), _ => Err("expected a JSON object".into()) }
    }
    pub fn array(&self) -> Result<&[Json], String> {
        match self { Self::Array(v) => Ok(v), _ => Err("expected a JSON array".into()) }
    }
    pub fn string(&self) -> Result<&str, String> {
        match self { Self::Text(v) => Ok(v), _ => Err("expected text".into()) }
    }
    pub fn integer(&self, min: u32, max: u32) -> Result<u32, String> {
        match self {
            Self::Number(v) if v.is_finite() && v.fract() == 0.0 && *v >= min as f64 && *v <= max as f64 => Ok(*v as u32),
            _ => Err(format!("expected an integer between {min} and {max}")),
        }
    }
}
pub fn field<'a>(v: &'a BTreeMap<String, Json>, key: &str) -> Result<&'a Json, String> {
    v.get(key).ok_or_else(|| format!("missing '{key}'"))
}
pub fn only(v: &BTreeMap<String, Json>, allowed: &[&str]) -> Result<(), String> {
    for key in v.keys() {
        if !allowed.contains(&key.as_str()) { return Err(format!("unsupported field '{key}'")); }
    }
    Ok(())
}
struct Reader<'a> { data: &'a [u8], pos: usize }
impl Reader<'_> {
    fn ws(&mut self) { while self.data.get(self.pos).map_or(false, |b| b.is_ascii_whitespace()) { self.pos += 1; } }
    fn byte(&mut self, expected: u8) -> Result<(), String> {
        if self.data.get(self.pos) != Some(&expected) { return Err(format!("expected '{}' at byte {}", expected as char, self.pos)); }
        self.pos += 1; Ok(())
    }
    fn hex4(&mut self) -> Result<u16, String> {
        let mut value = 0u16;
        for _ in 0..4 {
            let b = *self.data.get(self.pos).ok_or("incomplete Unicode escape")?;
            value = value.checked_mul(16).unwrap() + match b {
                b'0'..=b'9' => (b - b'0') as u16, b'a'..=b'f' => (b - b'a' + 10) as u16,
                b'A'..=b'F' => (b - b'A' + 10) as u16, _ => return Err("invalid Unicode escape".into()),
            };
            self.pos += 1;
        }
        Ok(value)
    }
    fn string(&mut self) -> Result<String, String> {
        self.byte(b'"')?;
        let mut output = String::new();
        let mut start = self.pos;
        loop {
            let b = *self.data.get(self.pos).ok_or("unterminated string")?;
            if b == b'"' || b == b'\\' {
                output.push_str(std::str::from_utf8(&self.data[start..self.pos]).map_err(|_| "invalid UTF-8")?);
                self.pos += 1;
                if b == b'"' { return Ok(output); }
                let escaped = *self.data.get(self.pos).ok_or("truncated escape")?;
                self.pos += 1;
                match escaped {
                    b'"' => output.push('"'), b'\\' => output.push('\\'), b'/' => output.push('/'),
                    b'b' => output.push('\u{0008}'), b'f' => output.push('\u{000c}'),
                    b'n' => output.push('\n'), b'r' => output.push('\r'), b't' => output.push('\t'),
                    b'u' => {
                        let high = self.hex4()?;
                        let scalar = if (0xd800..=0xdbff).contains(&high) {
                            self.byte(b'\\')?; self.byte(b'u')?;
                            let low = self.hex4()?;
                            if !(0xdc00..=0xdfff).contains(&low) { return Err("invalid surrogate pair".into()); }
                            0x10000 + (((high as u32 - 0xd800) << 10) | (low as u32 - 0xdc00))
                        } else {
                            if (0xdc00..=0xdfff).contains(&high) { return Err("lone low surrogate".into()); }
                            high as u32
                        };
                        output.push(char::from_u32(scalar).ok_or("invalid Unicode scalar")?);
                    }
                    _ => return Err("invalid string escape".into()),
                }
                start = self.pos;
            } else {
                if b < 0x20 { return Err("unescaped control character".into()); }
                self.pos += 1;
            }
        }
    }
    fn value(&mut self, depth: u8) -> Result<Json, String> {
        if depth > 32 { return Err("JSON nesting too deep".into()); }
        self.ws();
        let b = *self.data.get(self.pos).ok_or("unexpected end of JSON")?;
        match b {
            b'{' => {
                self.pos += 1; self.ws(); let mut object = BTreeMap::new();
                if self.data.get(self.pos) == Some(&b'}') { self.pos += 1; return Ok(Json::Object(object)); }
                loop {
                    self.ws(); let key = self.string()?;
                    self.ws(); self.byte(b':')?; let value = self.value(depth + 1)?;
                    if object.insert(key, value).is_some() { return Err("duplicate object key".into()); }
                    self.ws();
                    match self.data.get(self.pos) {
                        Some(b',') => self.pos += 1, Some(b'}') => { self.pos += 1; break; }
                        _ => return Err("expected ',' or '}'".into()),
                    }
                }
                Ok(Json::Object(object))
            }
            b'[' => {
                self.pos += 1; self.ws(); let mut list = Vec::new();
                if self.data.get(self.pos) == Some(&b']') { self.pos += 1; return Ok(Json::Array(list)); }
                loop {
                    list.push(self.value(depth + 1)?); self.ws();
                    match self.data.get(self.pos) {
                        Some(b',') => self.pos += 1, Some(b']') => { self.pos += 1; break; }
                        _ => return Err("expected ',' or ']'".into()),
                    }
                }
                Ok(Json::Array(list))
            }
            b'"' => self.string().map(Json::Text),
            b't' | b'f' | b'n' => {
                let (word, value) = match b {
                    b't' => (b"true".as_slice(), Json::Bool),
                    b'f' => (b"false".as_slice(), Json::Bool),
                    _ => (b"null".as_slice(), |_| Json::Null),
                };
                if !self.data[self.pos..].starts_with(word) { return Err("invalid keyword".into()); }
                self.pos += word.len();
                Ok(match word { b"true" => Json::Bool(true), b"false" => Json::Bool(false), _ => Json::Null })
            }
            b'-' | b'0'..=b'9' => {
                let start = self.pos;
                if b == b'-' { self.pos += 1; }
                match self.data.get(self.pos) {
                    Some(b'0') => self.pos += 1,
                    Some(b'1'..=b'9') => {
                        self.pos += 1;
                        while self.data.get(self.pos).map_or(false, |x| x.is_ascii_digit()) { self.pos += 1; }
                    }
                    _ => return Err("invalid number".into()),
                }
                if self.data.get(self.pos) == Some(&b'.') {
                    self.pos += 1;
                    if !self.data.get(self.pos).map_or(false, |x| x.is_ascii_digit()) { return Err("invalid fraction".into()); }
                    while self.data.get(self.pos).map_or(false, |x| x.is_ascii_digit()) { self.pos += 1; }
                }
                if matches!(self.data.get(self.pos), Some(b'e') | Some(b'E')) {
                    self.pos += 1;
                    if matches!(self.data.get(self.pos), Some(b'+') | Some(b'-')) { self.pos += 1; }
                    if !self.data.get(self.pos).map_or(false, |x| x.is_ascii_digit()) { return Err("invalid exponent".into()); }
                    while self.data.get(self.pos).map_or(false, |x| x.is_ascii_digit()) { self.pos += 1; }
                }
                let raw = std::str::from_utf8(&self.data[start..self.pos]).map_err(|_| "invalid number")?;
                Ok(Json::Number(raw.parse().map_err(|_| "invalid number")?))
            }
            _ => Err(format!("unexpected byte at position {}", self.pos)),
        }
    }
}
pub fn parse(data: &[u8]) -> Result<Json, String> {
    if data.len() > 65536 { return Err("manifest exceeds 64 KiB".into()); }
    let mut r = Reader { data, pos: 0 };
    let value = r.value(0)?; r.ws();
    if r.pos != r.data.len() { return Err("trailing JSON data".into()); }
    Ok(value)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn parses_escapes_and_rejects_duplicates() {
        let parsed = parse(br#"{"a":"\uD83D\uDE00","b":[1,true,null]}"#).unwrap();
        assert_eq!(field(parsed.object().unwrap(), "a").unwrap().string().unwrap(), "😀");
        assert!(parse(br#"{"a":1,"a":2}"#).is_err());
        assert!(parse(br#"{"a":"\uD800"}"#).is_err());
        assert!(parse(br#"{"a":01}"#).is_err());
    }
}
