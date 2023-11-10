use std::env;
use std::path::PathBuf;

fn main() {
    // Use pkg-config to find the urcrypt library.
    let urcrypt = pkg_config::Config::new().atleast_version("0.1.0").probe("liburcrypt-0").unwrap();


    for path in urcrypt.link_paths.iter() {
        println!("cargo:rustc-link-search={}", path.display());

        eprintln!("cargo:rustc-link-search={}", path.display());
    }

    for path in urcrypt.libs.iter() {
        println!("cargo:rustc-link-lib={}", path);
        eprintln!("cargo:rustc-link-lib={}", path);
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes.
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default();

    // Tell bindgen where to find the include file
    for path in urcrypt.include_paths.iter() {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = builder
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
