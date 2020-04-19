extern crate bindgen;

fn main() {

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/lib_main.o");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=vorbisfile");
    println!("cargo:rustc-link-lib=vorbis");
    println!("cargo:rustc-link-lib=ogg");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=framework=OpenGL");

    cc::Build::new()
        .object("src/lib_main.o")
        .object("src/screen.o")
        .object("src/input.o")
        .object("src/hostcall.o")
        .object("src/keymap.o")
        .object("src/shortcut.o")
        .object("src/audio.o")
        .object("fe2.part1.o")
        .object("fe2.part2.o")
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
        .clang_arg("-L/usr/local/lib -lSDL2")
        .clang_arg("-I/usr/local/include/SDL2")
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
