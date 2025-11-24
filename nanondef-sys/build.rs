extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // config should be taken from the workspace root
    let config = crate_dir.join("cbindgen.toml");

    // ensure rebuilds when these change:
    println!("cargo:rerun-if-changed={}/src", crate_dir.display());
    println!("cargo:rerun-if-changed={}", config.display());

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(cbindgen::Config::from_file(&config).unwrap())
        .generate()
        .map_or_else(
            |error| match error {
                cbindgen::Error::ParseSyntaxError { .. } => {}
                e => panic!("{:?}", e),
            },
            |bindings| {
                bindings.write_to_file("target/include/bindings.h");
            },
        );
}

fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
