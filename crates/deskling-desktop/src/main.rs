use std::ffi::{c_char, CStr, CString};
use std::process::ExitCode;
use deskling_core::{parse_args, validate_link_url, Command, HELP, VERSION};

extern "C" {
    fn deskling_run(argc: i32, argv: *mut *mut c_char, link_url: *const c_char,
        reset_position: bool, smoke_test: bool, capture_path: *const c_char, pack_path: *const c_char) -> i32;
}

/// Validate URLs edited in the Qt preferences with the same rules as the CLI.
/// The caller must provide either null or a valid NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn deskling_validate_link_url(url: *const c_char) -> bool {
    if url.is_null() { return false; }
    // No unwrap, indexing, or panic path may cross the C ABI boundary.
    match CStr::from_ptr(url).to_str() {
        Ok(text) => validate_link_url(text).is_ok(),
        Err(_) => false,
    }
}

fn run() -> Result<i32, String> {
    let args = std::env::args_os().skip(1).map(|arg| arg.into_string()
        .map_err(|_| "Arguments must be valid UTF-8. Use a UTF-8 path or URL.".to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let options = match parse_args(args).map_err(|e| e.to_string())? {
        Command::Help => { print!("{HELP}"); return Ok(0); }
        Command::Version => { println!("deskling-desktop {VERSION}"); return Ok(0); }
        Command::Run(options) => options,
    };
    let as_c_string = |value: Option<String>| -> Result<Option<CString>, String> {
        value.map(|s| CString::new(s).map_err(|_| "Arguments cannot contain NUL bytes.".to_string())).transpose()
    };
    let link_url = as_c_string(options.link_url)?;
    let capture_path = as_c_string(options.capture_path)?;
    let pack_path = as_c_string(options.pack_path)?;
    // Qt may mutate argc/argv. Supply writable owned storage for argv[0], and
    // never forward our flags to Qt (including untrusted URL/path arguments).
    let mut app_name = b"deskling-desktop\0".to_vec();
    let mut argv = [app_name.as_mut_ptr().cast::<c_char>(), std::ptr::null_mut()];
    let ptr = |s: &Option<CString>| s.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());
    // All backing strings and argv remain alive until the Qt event loop exits.
    Ok(unsafe { deskling_run(1, argv.as_mut_ptr(), ptr(&link_url), options.reset_position,
        options.smoke_test, ptr(&capture_path), ptr(&pack_path)) })
}

fn main() -> ExitCode {
    match run() {
        Ok(0) => ExitCode::SUCCESS,
        Ok(code) => ExitCode::from(if (1..=255).contains(&code) { code as u8 } else { 1 }),
        Err(error) => { eprintln!("deskling-desktop: {error}"); ExitCode::from(2) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ffi_validator_handles_null_and_invalid_utf8() {
        assert!(!unsafe { deskling_validate_link_url(std::ptr::null()) });
        let invalid_utf8 = [0xffu8, 0];
        assert!(!unsafe { deskling_validate_link_url(invalid_utf8.as_ptr().cast()) });
    }
    #[test]
    fn ffi_validator_agrees_with_core() {
        let good = b"https://example.com/user-supplied-route\0";
        let bad = b"https://user@example.com/\0";
        assert!(unsafe { deskling_validate_link_url(good.as_ptr().cast()) });
        assert!(!unsafe { deskling_validate_link_url(bad.as_ptr().cast()) });
    }
}
