#[cfg(feature = "build_bindings")]
use std::env;
#[cfg(feature = "build_bindings")]
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .include("minimp3/")
        .file("minimp3.c")
        .define("MINIMP3_IMPLEMENTATION", None)
        .compile("minimp3");

    // re-enable if bindings have not been created yet
    // for easy of cross compilation we take this out of build.rs
    #[cfg(feature = "build_bindings")]
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // .header("minimp3/minimp3.h")
        // .header("minimp3/minimp3_ex.h")
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    #[cfg(feature = "build_bindings")]
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    #[cfg(feature = "build_bindings")]
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
