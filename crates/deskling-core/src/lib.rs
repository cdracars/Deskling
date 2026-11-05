//! Dependency-free configuration for the Deskling desktop companion.
//! No network, account, cookie, or session access is performed here.

use std::fmt;

pub const VERSION: &str = "0.1.0";
pub const HELP: &str = "Deskling desktop companion 0.1.0 (Linux X11)\n\nUsage: deskling-desktop [OPTIONS]\n\n  --pack PATH          Load a local JSON sprite-pack manifest\n  --link-url URL       Optional HTTPS browser shortcut (no credentials or secrets)\n                      DNS hosts only; no ports, queries, or fragments\n  --reset-position    Ignore the saved window position on this launch\n  --smoke-test        Run the short graphical startup test and exit\n  --capture PATH      Save a screenshot during --smoke-test\n  --help              Show this help\n  --version           Show the version\n\nWithout --pack, show the built-in diagnostic geometric shape.\nNo link is configured by default. Never supply private tokens in URLs.\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}
impl std::error::Error for Error {}

/// Accept explicit lowercase `https://` and an ASCII DNS hostname with at least
/// two labels and an alphabetic start to the final label. Host matching is case-insensitive; trailing dots, ports, IP
/// literals, credentials, queries and fragments are deliberately unsupported.
/// Paths are opaque RFC 3986 path syntax with valid percent escapes. Escaped
/// controls, spaces and backslashes are rejected. This is syntax validation,
/// not a trust check or secret detector: callers must not provide private tokens.
pub fn validate_link_url(url: &str) -> Result<(), Error> {
    let fail = || Error("Use an https:// URL with an ASCII DNS hostname and no port, credentials, query, fragment, spaces, or backslashes. Never include private tokens.".into());
    if !url.is_ascii() || url.bytes().any(|b| b <= 0x20 || b >= 0x7f || b == b'\\') {
        return Err(fail());
    }
    let rest = url.strip_prefix("https://").ok_or_else(fail)?;
    let (host, path) = rest.split_once('/').map_or((rest, ""), |(h, p)| (h, p));
    if host.len() > 253 || !host.contains('.') || host.parse::<std::net::Ipv4Addr>().is_ok() {
        return Err(fail());
    }
    for label in host.split('.') {
        if label.is_empty() || label.len() > 63
            || !label.as_bytes()[0].is_ascii_alphanumeric()
            || !label.as_bytes()[label.len() - 1].is_ascii_alphanumeric()
            || !label.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
            return Err(fail());
        }
    }
    // Require an alphabetic first byte in the final label. This also rejects
    // alternate numeric IP notations such as hexadecimal IPv4 components.
    if !host.rsplit('.').next().unwrap_or("").bytes().next().map_or(false, |b| b.is_ascii_alphabetic()) {
        return Err(fail());
    }
    if path.contains(['?', '#']) { return Err(fail()); }
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'%' {
            let hex = |x: u8| -> Option<u8> {
                match x {
                    b'0'..=b'9' => Some(x - b'0'),
                    b'a'..=b'f' => Some(x - b'a' + 10),
                    b'A'..=b'F' => Some(x - b'A' + 10),
                    _ => None,
                }
            };
            let decoded = bytes.get(i + 1).and_then(|a| hex(*a)).zip(bytes.get(i + 2).and_then(|b| hex(*b)))
                .map(|(a, b)| a * 16 + b).ok_or_else(fail)?;
            if decoded <= 0x20 || decoded == 0x7f || decoded == b'\\' { return Err(fail()); }
            i += 3;
        } else {
            // RFC 3986 pchar plus path separators. Quotes, brackets and HTML
            // delimiters must be percent encoded rather than silently repaired.
            if !(b.is_ascii_alphanumeric() || b"-._~!$&'()*+,;=:@/".contains(&b)) { return Err(fail()); }
            i += 1;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Options {
    pub link_url: Option<String>,
    pub pack_path: Option<String>,
    pub reset_position: bool,
    pub smoke_test: bool,
    pub capture_path: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command { Run(Options), Help, Version }

/// Parse arguments excluding argv[0]. Reject duplicate options and positional
/// arguments, so mistakes do not silently alter launch behavior.
pub fn parse_args<I, S>(args: I) -> Result<Command, Error>
where I: IntoIterator<Item = S>, S: Into<String> {
    let mut args = args.into_iter().map(Into::into);
    let mut options = Options::default();
    let mut help = false;
    let mut version = false;
    while let Some(arg) = args.next() {
        let duplicate = || Error(format!("Option {arg} was supplied more than once; use it only once."));
        match arg.as_str() {
            "--link-url" | "--capture" | "--pack" => {
                let target = match arg.as_str() { "--link-url" => &mut options.link_url, "--pack" => &mut options.pack_path, _ => &mut options.capture_path };
                if target.is_some() { return Err(duplicate()); }
                let value = args.next().ok_or_else(|| Error(format!("{arg} needs a value. Run deskling-desktop --help for usage.")))?;
                if value.is_empty() || value.starts_with("--") || value.contains('\0') {
                    return Err(Error(format!("{arg} needs a nonempty value (not another option).")));
                }
                if arg == "--link-url" { validate_link_url(&value)?; }
                *target = Some(value);
            }
            "--reset-position" => { if options.reset_position { return Err(duplicate()); } options.reset_position = true; }
            "--smoke-test" => { if options.smoke_test { return Err(duplicate()); } options.smoke_test = true; }
            "--help" => { if help { return Err(duplicate()); } help = true; }
            "--version" => { if version { return Err(duplicate()); } version = true; }
            _ => return Err(Error(format!("Unknown argument {arg:?}. Run deskling-desktop --help for supported options."))),
        }
    }
    if help { return Ok(Command::Help); }
    if version { return Ok(Command::Version); }
    if options.capture_path.is_some() && !options.smoke_test {
        return Err(Error("--capture requires --smoke-test. Add --smoke-test or remove --capture.".into()));
    }
    Ok(Command::Run(options))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_dns_hosts_and_opaque_paths() {
        for url in ["https://example.com", "https://EXAMPLE.com/", "https://docs.example.org/path", "https://my-host.example/a%2Fb", "https://example.com/%E2%9C%93", "https://example.com/a:b@c", "https://example.com/a-._~!$&'()*+,;="] {
            assert!(validate_link_url(url).is_ok(), "{url}");
        }
    }
    #[test]
    fn rejects_unsupported_authorities_and_schemes() {
        for url in ["", "example.com", "http://example.com/", "HTTPS://example.com/", "https://user@example.com/", "https://example.com:443/", "https://example.com./", "https://example.com%2Fevil/", "https://localhost/", "https://127.0.0.1/", "https://127.1/", "https://0x7f.0x1/", "https://[::1]/", "https://-bad.example/", "https://bad-.example/", "https://a..example/", "https://a_b.example/", "https://é.example/", "https://example.com?token=x", "https://example.com#x", "javascript:alert(1)"] {
            assert!(validate_link_url(url).is_err(), "{url}");
        }
        assert!(validate_link_url(&format!("https://{}.com", "a".repeat(64))).is_err());
        assert!(validate_link_url(&format!("https://{}.com", "aaaa.".repeat(60))).is_err());
    }
    #[test]
    fn rejects_unsafe_or_ambiguous_path_text() {
        for path in ["/?token=x", "/#fragment", "/space here", "/a\\b", "/a\nb", "/a\0b", "/a\tb", "/é", "/%", "/%0", "/%GG", "/%00", "/%0a", "/%0D", "/%20", "/%5c", "/%7F", "/<x>", "/\"x\"", "/[x]"] {
            assert!(validate_link_url(&format!("https://example.com{path}")).is_err(), "{path:?}");
        }
    }
    #[test]
    fn defaults_to_no_pack_or_link() {
        assert_eq!(parse_args(Vec::<String>::new()).unwrap(), Command::Run(Options::default()));
    }
    #[test]
    fn parses_all_run_flags() {
        assert_eq!(parse_args(["--pack", "./my pack/pack.json", "--capture", "/tmp/deskling shot.png", "--link-url", "https://example.com/user-route", "--smoke-test", "--reset-position"]).unwrap(), Command::Run(Options { link_url: Some("https://example.com/user-route".into()), pack_path: Some("./my pack/pack.json".into()), reset_position: true, smoke_test: true, capture_path: Some("/tmp/deskling shot.png".into()) }));
    }
    #[test]
    fn help_and_version_are_commands() {
        assert_eq!(parse_args(["--help"]).unwrap(), Command::Help);
        assert_eq!(parse_args(["--version"]).unwrap(), Command::Version);
    }
    #[test]
    fn rejects_missing_values_unknown_and_duplicate_flags() {
        for args in [vec!["--link-url"], vec!["--capture"], vec!["--pack"], vec!["--pack", ""], vec!["--pack", "--help"], vec!["--pack", "a\0b"], vec!["--pack", "a", "--pack", "b"], vec!["--link-url", ""], vec!["--link-url", "--help"], vec!["--link-url", "http://example.com"], vec!["--capture", "a.png"], vec!["--capture", "a\0b", "--smoke-test"], vec!["--reset-position", "--reset-position"], vec!["--smoke-test", "--smoke-test"], vec!["--link-url", "https://example.com", "--link-url", "https://example.com"], vec!["--capture", "a", "--capture", "b", "--smoke-test"], vec!["--help", "--help"], vec!["--version", "--version"], vec!["--wat"], vec!["positional"], vec!["--link-url=https://example.com"]] {
            assert!(parse_args(args.clone()).is_err(), "{args:?}");
        }
    }
}
