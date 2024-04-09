#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
use std::{env, path::Path};

use anyhow::Result;
use bindgen::builder;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=prebuilt");

    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    let target = env::var("TARGET")?;
    match target.as_str() {
        // android
        "aarch64-linux-android" => {
            println!("cargo:rustc-link-search=native=prebuilt/android/arm64-v8a")
        }
        "arm-linux-androideabi" | "armv7-linux-androideabi" => {
            println!("cargo:rustc-link-search=native=prebuilt/android/armeabi-v7a")
        }
        "i686-linux-android" => println!("cargo:rustc-link-search=native=prebuilt/android/x86"),
        "x86_64-linux-android" => {
            println!("cargo:rustc-link-search=native=prebuilt/android/x86_64")
        }
        // macos
        "aarch64-apple-darwin" => println!("cargo:rustc-link-search=native=prebuilt/macos/arm64"),
        "arm64e-apple-darwin" => println!("cargo:rustc-link-search=native=prebuilt/macos/arm64e"),
        "i686-apple-darwin" | "x86_64h-apple-darwin" => {
            println!("cargo:rustc-link-search=native=prebuilt/macos/universal")
        }
        "x86_64-apple-darwin" => println!("cargo:rustc-link-search=native=prebuilt/macos/x86_64"),
        // ios
        "aarch64-apple-ios" | "aarch64-apple-ios-sim" | "aarch64-apple-ios-macabi" => {
            println!("cargo:rustc-link-search=native=prebuilt/ios/arm64")
        }
        "arm64e-apple-ios" => println!("cargo:rustc-link-search=native=prebuilt/ios/arm64e"),
        "armv7s-apple-ios" | "i386-apple-ios" | "x86_64-apple-ios" | "x86_64-apple-ios-macabi" => {
            println!("cargo:rustc-link-search=native=prebuilt/ios/universal")
        }
        // linux
        "aarch64-unknown-linux-gnu"
        | "aarch64-unknown-linux-ohos"
        | "aarch64-unknown-linux-gnu_ilp32"
        | "aarch64-unknown-linux-musl"
        | "aarch64_be-unknown-linux-gnu_ilp32"
        | "aarch64_be-unknown-linux-gnu" => {
            println!("cargo:rustc-link-search=native=prebuilt/linux/arm64")
        }
        "arm-unknown-linux-gnueabi"
        | "arm-unknown-linux-gnueabihf"
        | "arm-unknown-linux-musleabi"
        | "arm-unknown-linux-musleabihf"
        | "armv4t-unknown-linux-gnueabi"
        | "armeb-unknown-linux-gnueabi"
        | "armv5te-unknown-linux-musleabi"
        | "armv5te-unknown-linux-gnueabi"
        | "armv5te-unknown-linux-uclibceabi"
        | "armv7-unknown-linux-gnueabi"
        | "armv7-unknown-linux-gnueabihf"
        | "armv7-unknown-linux-musleabihf"
        | "armv7-unknown-linux-musleabi"
        | "armv7-unknown-linux-uclibceabi"
        | "armv7-unknown-linux-ohos"
        | "armv7-unknown-linux-uclibceabihf" => {
            println!("cargo:rustc-link-search=native=prebuilt/linux/arm")
        }
        "i586-unknown-linux-musl"
        | "i586-unknown-linux-gnu"
        | "i686-unknown-linux-musl"
        | "i686-unknown-linux-gnu" => println!("cargo:rustc-link-search=native=prebuilt/linux/x86"),
        "x86_64-unikraft-linux-musl"
        | "x86_64-unknown-linux-gnu"
        | "x86_64-unknown-linux-gnux32"
        | "x86_64-unknown-linux-ohos"
        | "x86_64-unknown-linux-musl" => {
            println!("cargo:rustc-link-search=native=prebuilt/linux/x86_64")
        }
        _ => panic!("Unsupported target: {target}"),
    }

    /* #[cfg(target_os = "ios")]
    {
        println!("cargo:rustc-link-search=native=prebuilt/ios/arm64");
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/macos/arm64");
        }

        #[cfg(target_arch = "x86_64")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/macos/x86_64");
        }
    }

    #[cfg(target_os = "linux")]
    {
        #[cfg(target_arch = "arm")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/linux/arm");
        }

        #[cfg(target_arch = "aarch64")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/linux/arm64");
        }

        #[cfg(target_arch = "x86")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/linux/x86");
        }

        #[cfg(target_arch = "x86_64")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/linux/x86_64");
        }
    }

    #[cfg(target_os = "android")]
    {
        #[cfg(target_arch = "aarch64")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/android/arm64-v8a");
        }

        #[cfg(target_arch = "arm")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/android/armeabi-v7a");
        }

        #[cfg(target_arch = "x86")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/android/x86");
        }

        #[cfg(target_arch = "x86_64")]
        {
            println!("cargo:rustc-link-search=native=prebuilt/android/x86_64");
        }
    } */

    println!("cargo:rustc-link-search=native=prebuilt/android/arm64-v8a");
    println!("cargo:rustc-link-lib=static=dobby");
    println!("cargo:rustc-link-lib=dylib=c++");

    let binding_path = out_dir.join("bindings.rs");
    let bindings = builder()
        .header("prebuilt/dobby.h")
        .allowlist_function("DobbyHook")
        .allowlist_function("DobbyDestroy")
        .allowlist_function("DobbyCodePatch")
        .allowlist_function("DobbyGetVersion")
        .allowlist_function("DobbySymbolResolver")
        .allowlist_function("DobbyImportTableReplace")
        .generate()?;
    bindings.write_to_file(binding_path)?;

    Ok(())
}
