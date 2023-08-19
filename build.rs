use std::{env, error::Error, path::Path, process::Command};

use bindgen::builder;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=Dobby");

    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    println!("cargo:warning='Since the dobby build commands may download new llvm & clang & ndk, this can be super slow'");

    #[cfg(target_os = "ios")]
    {
        Command::new("python")
            .current_dir("Dobby")
            .args(["--platform=iphoneos", "--arch=all"])
            .spawn()?
            .wait()?;

        println!("cargo:rustc-link-search=native=Dobby/build/ios/arm64");
        println!("cargo:rustc-link-lib=static=dobby");
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("python")
            .current_dir("Dobby")
            .args(["--platform=macos", "--arch=all"])
            .spawn()?
            .wait()?;

        #[cfg(target_arch = "aarch64")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/macos/arm64");
            println!("cargo:rustc-link-lib=static=dobby");
        }

        #[cfg(target_arch = "x86_64")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/macos/x86_64");
            println!("cargo:rustc-link-lib=static=dobby");
        }
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("sh")
            .current_dir("Dobby")
            .arg("scripts/setup_linux_cross_compile.sh")
            .spawn()?
            .wait()?;
        Command::new("python3")
            .current_dir("Dobby")
            .arg("scripts/platform_builder.py")
            .args([
                "--platform=linux",
                "--arch=all",
                "--cmake_dir=$HOME/opt/cmake-3.25.2",
                "--llvm_dir=$HOME/opt/llvm-15.0.6",
            ])
            .spawn()?
            .wait()?;

        #[cfg(target_arch = "arm")]
        {
            println!("cargo:rustc-link-search=native=dobby/linux/arm");
            println!("cargo:rustc-link-lib=static=dobby");
        }

        #[cfg(target_arch = "aarch64")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/linux/arm64");
            println!("cargo:rustc-link-lib=static=dobby");
        }

        #[cfg(target_arch = "x86")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/linux/x86");
            println!("cargo:rustc-link-lib=static=dobby");
        }

        #[cfg(target_arch = "x86_64")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/linux/x86_64");
            println!("cargo:rustc-link-lib=static=dobby");
        }
    }

    #[cfg(target_os = "android")]
    {
        Command::new("sh")
            .current_dir("Dobby")
            .arg("scripts/setup_linux_cross_compile.sh")
            .spawn()?
            .wait()?;
        Command::new("python3")
            .current_dir("Dobby")
            .arg("scripts/platform_builder.py")
            .args([
                "--platform=android",
                "--arch=all",
                "--cmake_dir=$HOME/opt/cmake-3.25.2",
                "--llvm_dir=$HOME/opt/llvm-15.0.6",
                "--android_ndk_dir=$HOME/opt/ndk-r25b",
            ])
            .spawn()?
            .wait()?;

        #[cfg(target_arch = "aarch64")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/android/arm64-v8a");
            println!("cargo:rustc-link-lib=static=dobby");
        }

        #[cfg(target_arch = "arm")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/android/armeabi-v7a");
            println!("cargo:rustc-link-lib=static=dobby");
        }

        #[cfg(target_arch = "x86")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/android/x86");
            println!("cargo:rustc-link-lib=static=dobby");
        }

        #[cfg(target_arch = "x86_64")]
        {
            println!("cargo:rustc-link-search=native=Dobby/build/android/x86_64");
            println!("cargo:rustc-link-lib=static=dobby");
        }
    }

    let binding_path = out_dir.join("bindings.rs");

    let bindings = builder()
        .header("Dobby/include/dobby.h")
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
