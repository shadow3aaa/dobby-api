use std::{env, error::Error, path::Path};

use bindgen::builder;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=prebuilt");

    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

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
