use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=MOS_BUILD");
    if env::var_os("MOS_BUILD").is_some() {
        println!("cargo:rustc-cfg=mos_build");
    }
    if env::var_os("MOS_TEST").is_some() {
        println!("cargo:rustc-cfg=mos_test");
    }
}
