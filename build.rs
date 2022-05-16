extern crate bindgen;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/lib_main.o");
    println!("cargo:rerun-if-changed=src/lib_main.h");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=framework=OpenGL");
    println!("cargo:rustc-link-arg=-L/opt/homebrew/lib");
    println!("cargo:rustc-link-arg=-lm");

    cc::Build::new()
        .object("src/screen.o")
        .object("src/input.o")
        .object("src/hostcall.o")
        .object("fe2.s.o")
        .debug(true)
        .compile("fe2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        .trust_clang_mangling(false)
        .clang_arg("-I/usr/local/include/SDL2")
        .clang_arg("-I/opt/homebrew/include/SDL2")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
