use std::{env, error::Error, path::Path};

use bindgen::builder;
use chrono::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=Dobby");

    let today = Local::now();
    let version = today.format("\"Dobby-%Y%m%d\"").to_string();

    cc::Build::new()
        .define("__DOBBY_BUILD_VERSION__", &*version)
        .include("Dobby/external")
        .include("Dobby/include")
        .include("Dobby/source")
        .include("Dobby/external/logging/")
        .include("Dobby/source/Backend/KernelMode")
        .include("Dobby/source/Backend/UserMode")
        .file("Dobby/source/dobby.cpp")
        .compile("libdobby.a");

    let binding_path = Path::new(&env::var("OUT_DIR")?).join("bindings.rs");

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
