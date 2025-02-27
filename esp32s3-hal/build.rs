use std::{env, fs::File, io::Write, path::PathBuf};

#[cfg(not(feature = "direct-boot"))]
fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("ld/memory.x"))
        .unwrap();

    File::create(out.join("alias.x"))
        .unwrap()
        .write_all(include_bytes!("ld/rom.x"))
        .unwrap();

    File::create(out.join("hal-defaults.x"))
        .unwrap()
        .write_all(include_bytes!("ld/hal-defaults.x"))
        .unwrap();

    File::create(out.join("esp32s3.x"))
        .unwrap()
        .write_all(include_bytes!("ld/esp32s3.x"))
        .unwrap();

    File::create(out.join("linkall.x"))
        .unwrap()
        .write_all(include_bytes!("ld/linkall.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=ld/memory.x");
}

#[cfg(feature = "direct-boot")]
fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("ld/db-memory.x"))
        .unwrap();

    File::create(out.join("alias.x"))
        .unwrap()
        .write_all(include_bytes!("ld/rom.x"))
        .unwrap();

    File::create(out.join("hal-defaults.x"))
        .unwrap()
        .write_all(include_bytes!("ld/hal-defaults.x"))
        .unwrap();

    File::create(out.join("esp32s3.x"))
        .unwrap()
        .write_all(include_bytes!("ld/db-esp32s3.x"))
        .unwrap();

    File::create(out.join("linkall.x"))
        .unwrap()
        .write_all(include_bytes!("ld/linkall.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=ld/memory.x");
}
