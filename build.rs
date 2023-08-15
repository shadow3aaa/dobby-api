use std::{env, error::Error, path::Path, process::Command};

use bindgen::builder;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=Dobby");

    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    Command::new("cmake")
        .args(["-S", "Dobby"])
        .args(["-B", &out_dir.display().to_string()])
        .arg(r#"-DCMAKE_C_FLAGS="-Os -static-libstdc++ -flto -fmerge-all-constants -fno-exceptions -fomit-frame-pointer -fshort-enums -Wl,-O3,--lto-O3,--gc-sections,--as-needed,--icf=all,-z,norelro -w""#)
        .spawn()?
        .wait()?;

    Command::new("make")
        .current_dir(out_dir)
        .arg("-j4")
        .spawn()?
        .wait()?;

    // println!("cargo:warning=lib_path={}", out_dir.display());
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=dobby");

    let binding_path = out_dir.join("bindings.rs");

    let bindings = builder()
        .header("Dobby/include/dobby.h")
        .allowlist_function("DobbyHook")
        .allowlist_function("DobbyDestroy")
        .allowlist_function("DobbyGetVersion")
        .allowlist_function("DobbySymbolResolver")
        .allowlist_function("DobbyImportTableReplace")
        .generate()?;

    bindings.write_to_file(binding_path)?;

    Ok(())
}
