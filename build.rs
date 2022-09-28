fn main() {
    // workaround for ar issue
    // https://github.com/rust-bitcoin/rust-secp256k1/pull/254#issuecomment-879588601
    std::env::set_var("AR", "/usr/local/opt/llvm/bin/llvm-ar");

    // build w4on
    println!("cargo:rerun-if-changed=w4on/w4on.c");
    println!("cargo:rustc-link-lib=w4on");
    cc::Build::new().file("w4on/w4on.c").compile("w4on");

    // create binding
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .clang_arg("-Iw4on")
        // workaround for bindgen for wasm issue
        // https://github.com/rust-lang/rust-bindgen/issues/751#issuecomment-496891269
        .clang_arg("-fvisibility=default")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
