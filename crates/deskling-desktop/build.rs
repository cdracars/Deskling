//! Small std-only build bridge: no network downloads, CMake, or Rust Qt crates.
use std::{env, path::PathBuf, process::Command};

fn output(program: &str, args: &[&str]) -> String {
    let result = Command::new(program).args(args).output()
        .unwrap_or_else(|e| panic!("Cannot run {program}: {e}. See README.md prerequisites."));
    if !result.status.success() {
        panic!("{program} {} failed:\n{}", args.join(" "), String::from_utf8_lossy(&result.stderr));
    }
    String::from_utf8(result.stdout).expect("Build tool output must be UTF-8").trim().to_owned()
}

fn run(program: &str, args: &[String]) {
    let status = Command::new(program).args(args).status()
        .unwrap_or_else(|e| panic!("Cannot run {program}: {e}"));
    assert!(status.success(), "{program} failed. See README.md prerequisites.");
}

fn qt_tool(name: &str) -> String {
    let key = format!("QT_{}", name.to_uppercase());
    println!("cargo:rerun-if-env-changed={key}");
    if let Ok(path) = env::var(&key) { return path; }
    let dir = output("pkg-config", &["--variable=libexecdir", "Qt6Core"]);
    let candidate = PathBuf::from(dir).join(name);
    if candidate.is_file() { return candidate.to_string_lossy().into_owned(); }
    for base in ["/usr/lib/qt6/libexec", "/usr/lib64/qt6/libexec", "/usr/lib/x86_64-linux-gnu/qt6/libexec"] {
        let candidate = PathBuf::from(base).join(name);
        if candidate.is_file() { return candidate.to_string_lossy().into_owned(); }
    }
    panic!("Qt 6 {name} was not found. Install Qt 6 development tools or set {key}.");
}

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("linux") {
        panic!("Deskling 0.1 targets Linux/X11 only.");
    }
    for path in ["native/bridge.hpp", "native/bridge.cpp", "native/resources.qrc", "qml/Main.qml", "native/pack.cpp", "native/pack.hpp"] {
        println!("cargo:rerun-if-changed={path}");
    }
    for name in ["CXX", "AR", "PKG_CONFIG_PATH", "PKG_CONFIG_LIBDIR"] {
        println!("cargo:rerun-if-env-changed={name}");
    }
    let version = output("pkg-config", &["--modversion", "Qt6Core"]);
    let parts: Vec<u32> = version.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    assert!(parts.len() >= 2 && (parts[0], parts[1]) >= (6, 4), "Qt 6.4 or later is required, found {version}");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let moc = out.join("moc_bridge.cpp");
    let rcc = out.join("qrc_deskling.cpp");
    run(&qt_tool("moc"), &["native/bridge.hpp".into(), "-o".into(), moc.display().to_string()]);
    run(&qt_tool("rcc"), &["--name".into(), "deskling".into(), "native/resources.qrc".into(), "-o".into(), rcc.display().to_string()]);
    let flags = output("pkg-config", &["--cflags", "Qt6Core", "Qt6Gui", "Qt6Qml", "Qt6Quick"]);
    let cxx = env::var("CXX").unwrap_or_else(|_| "c++".into());
    let mut objects = Vec::new();
    for (index, source) in [PathBuf::from("native/bridge.cpp"), PathBuf::from("native/pack.cpp"), moc, rcc].iter().enumerate() {
        let object = out.join(format!("deskling_{index}.o"));
        let mut args: Vec<String> = flags.split_whitespace().map(String::from).collect();
        args.extend(["-std=c++17".into(), "-fPIC".into(), "-O2".into(), "-Wall".into(), "-Wextra".into(), "-Inative".into(), "-c".into(), source.display().to_string(), "-o".into(), object.display().to_string()]);
        run(&cxx, &args);
        objects.push(object.display().to_string());
    }
    let archive = out.join("libdeskling_qt.a");
    let mut args = vec!["crs".into(), archive.display().to_string()];
    args.extend(objects);
    run(&env::var("AR").unwrap_or_else(|_| "ar".into()), &args);
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=deskling_qt");
    let libs = output("pkg-config", &["--libs", "Qt6Core", "Qt6Gui", "Qt6Qml", "Qt6Quick"]);
    for flag in libs.split_whitespace() {
        if let Some(name) = flag.strip_prefix("-l") { println!("cargo:rustc-link-lib={name}"); }
        else if let Some(path) = flag.strip_prefix("-L") { println!("cargo:rustc-link-search=native={path}"); }
        else { println!("cargo:rustc-link-arg={flag}"); }
    }
    println!("cargo:rustc-link-lib=stdc++");
}
