fn main() {
    println!("cargo:rerun-if-changed=src/");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::generate(manifest_dir).expect("Could not generate header");
}
