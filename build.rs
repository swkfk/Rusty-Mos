use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=MOS_BUILD");
    match env::var_os("MOS_BUILD") {
        None => (),
        Some(_) => {
            println!("cargo:rustc-cfg=mos_build");
        }
    }
}
