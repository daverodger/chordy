use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // copy 'progression.json' data to build output dir
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("progressions.json");
    fs::copy("progressions.json", &dest_path).unwrap();
    println!("cargo:rerun-if-changed=data.json");
}