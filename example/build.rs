use oxide_enzyme::{FncInfo, CDIFFE_TYPE};
use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=../src/lib.rs");
 
    let entry_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let check_path = entry_path.join("enzyme-done");
    println!("cargo:rerun-if-changed={}", check_path.display());

    let fnc_x = FncInfo::new("testx",  vec![CDIFFE_TYPE::DFT_OUT_DIFF]);
    let fnc_2 = FncInfo::new("test2",  vec![CDIFFE_TYPE::DFT_OUT_DIFF]);
    oxide_enzyme::build(
        vec![fnc_x, fnc_2]
    );
}
