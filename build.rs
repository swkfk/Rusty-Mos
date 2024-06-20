use std::{env, fs, io::Write, path::Path};

const ENV_MOS_TEST: &str = "MOS_TEST";
const ENV_MOS_RUN_ENV: &str = "MOS_RUN_ENV";

const TEST_MOS_RUN_ENV: &str = "run_env";

const CFG_TEST_ITEM: &str = "ktest_item";

fn main() {
    println!("cargo:rerun-if-env-changed={}", ENV_MOS_TEST);
    println!("cargo:rerun-if-env-changed={}", ENV_MOS_RUN_ENV);

    match env::var_os(ENV_MOS_TEST) {
        None => (),
        Some(test) => {
            let test = test.into_string().unwrap();
            if test == TEST_MOS_RUN_ENV {
                handle_run_env();
            }
            println!("cargo:rustc-cfg={}=\"{}\"", CFG_TEST_ITEM, test);
        }
    }
}

fn handle_run_env() {
    let env_key = env::var_os(ENV_MOS_RUN_ENV).unwrap().into_string().unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut include_file = fs::File::create(Path::new(&out_dir).join("include.rs")).unwrap();
    let _ = include_file
        .write(
            "fn setup_env_run() {
    use crate::kern::env::env_create;
    use core::ptr::addr_of;
"
            .as_bytes(),
        )
        .unwrap();
    match env_key.as_str() {
        "ppa1" => {
            load_icode(&include_file, "ppa", "ppa1");
            create_env(&include_file, "ppa", 5);
            create_env(&include_file, "ppa", 5);
        }
        "ppa2" => {
            load_icode(&include_file, "ppa", "ppa2");
            create_env(&include_file, "ppa", 1);
        }
        "fktest1" => {
            load_icode(&include_file, "fktest", "fktest1");
            create_env(&include_file, "fktest", 1);
        }
        "fktest2" => {
            load_icode(&include_file, "fktest", "fktest2");
            create_env(&include_file, "fktest", 1);
        }
        "pingpong" => {
            load_icode(&include_file, "pingpong", "pingpong_strong");
            create_env(&include_file, "pingpong", 20);
        }
        "fs_check" => {
            load_icode(&include_file, "fs_check", "fs_check");
            create_env(&include_file, "fs_check", 1);
        }
        "mix_check" => {
            load_icode(&include_file, "mix_check", "mix_check");
            create_env(&include_file, "mix_check", 1);
        }
        "serv_check" => {
            load_icode(&include_file, "serv_check", "serv_check");
            load_icode(&include_file, "fs_serv", "fs_serv");
            create_env(&include_file, "serv_check", 1);
            create_env(&include_file, "fs_serv", 1);
        }
        "fs_strong_check" => {
            // Pay attention, the disk is different!
            load_icode(&include_file, "fs_strong_check", "fs_strong_check");
            load_icode(&include_file, "fs_serv", "fs_serv");
            create_env(&include_file, "fs_strong_check", 1);
            create_env(&include_file, "fs_serv", 1);
        }
        "mos" => {
            load_icode(&include_file, "user_icode", "icode");
            load_icode(&include_file, "fs_serv", "serv");
            create_env(&include_file, "user_icode", 1);
            create_env(&include_file, "fs_serv", 1);
        }
        _ => unreachable!(),
    }
    let _ = include_file.write("}\n".as_bytes()).unwrap();
}

fn load_icode(mut file: &fs::File, ident: &str, filename: &str) {
    let _ = file
        .write(
            format!(
                "    let {} = include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/ktests/bin/{}.b\"));\n",
                ident, filename
            )
            .as_bytes(),
        )
        .unwrap();
}

fn create_env(mut file: &fs::File, ident: &str, priority: u32) {
    let _ = file
        .write(format!("    unsafe {{ env_create(addr_of!(*{ident}) as *const u8, {ident}.len(), {priority}).unwrap(); }}\n").as_bytes())
        .unwrap();
}
