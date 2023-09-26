use std::env;
use std::path::PathBuf;

fn main() {
    // Use pkg-config to find the urcrypt library.
    let _urcrypt = pkg_config::Config::new().atleast_version("0.1.0").probe("liburcrypt-0").unwrap();

    for path in _urcrypt.link_paths.iter() {
        println!("cargo:rustc-link-search={}", path.display());
    }

    for path in _urcrypt.libs.iter() {
        println!("cargo:rustc-link-lib={}", path);
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes.
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
