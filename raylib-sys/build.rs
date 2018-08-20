extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    // Locate raylib binary release by version and platform
    let raylib_version = "2.0.0";
    let raylib_release_url = format!(
        "https://github.com/raysan5/raylib/releases/download/{}/raylib-{}-{}",
        raylib_version,
        raylib_version,
        release_suffix_for_target(&target)
    );

    // Download raylib binary release
    let _ = Command::new("curl")
        .current_dir(&out_dir)
        .args(&["-sSfLo", "raylib.tar.gz", &raylib_release_url])
        .status()
        .expect("Failed to execute curl.")
        .success() || panic!("Failed to download compiled raylib.");

    // Untar raylib binary release
    let _ = Command::new("tar")
        .current_dir(&out_dir)
        .args(&["-xzf", "raylib.tar.gz", "--strip-components=1"])
        .status()
        .expect("Failed to execute tar.")
        .success() || panic!("Failed to untar compiled raylib.");

    // Generate and write raylib bindings
    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", out_dir.join("include").display()))
        .constified_enum_module("*")
        .generate()
        .expect("Failed to generate bindings.")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write bindings.");

    // Read raylib.pc and print cargo metadata for linking to raylib
    env::set_var("PKG_CONFIG_PATH", out_dir.join("lib/pkgconfig"));
    pkg_config::Config::new()
        .atleast_version(raylib_version)
        .statik(true)
        .arg(format!("--define-variable=prefix={}", out_dir.display()))
        .probe("raylib")
        .unwrap();
}

fn release_suffix_for_target(target: &str) -> String {
    // TODO: Remove this when other platforms are supported and tested
    if !target.contains("linux") {
        panic!("Unsupported target `{}`.", target);
    }
    if target.contains("apple") {
        return String::from("macOS.tar.gz");
    } else if target.contains("linux") {
        if target.contains("x86_64") {
            return String::from("Linux-amd64.tar.gz");
        } else if target.contains("i686") {
            return String::from("Linux-i386.tar.gz");
        }
    } else if target.contains("windows") {
        let arch = if target.contains("x86_64") {
            "Win64"
        } else {
            "Win32"
        };
        if target.contains("gnu") {
            return format!("{}-mingw.zip", arch);
        } else if target.contains("msvc") {
            return format!("{}-msvc15.zip", arch);
        }
    }
    panic!("Unsupported target `{}`.", target);
}
