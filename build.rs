use std::env;

const ENV_MOS_TEST: &str = "MOS_TEST";
const ENV_MOS_RUN_ENV: &str = "MOS_RUN_ENV";

const TEST_MOS_RUN_ENV: &str = "env_run";

const CFG_TEST_ITEM: &str = "ktest_item";

fn main() {
    println!("cargo::rerun-if-env-changed={}", ENV_MOS_TEST);
    println!("cargo::rerun-if-env-changed={}", ENV_MOS_RUN_ENV);

    match env::var_os(ENV_MOS_TEST) {
        None => (),
        Some(test) => {
            let test = test.into_string().unwrap();
            if test == TEST_MOS_RUN_ENV {
                handle_run_env();
            }
            println!("cargo::rustc-cfg={}=\"{}\"", CFG_TEST_ITEM, test);
        }
    }
}

fn handle_run_env() {
    let env_key = env::var_os(ENV_MOS_RUN_ENV).unwrap().into_string().unwrap();
    match env_key.as_str() {
        "loop" => (),
        "qsort" => (),
        _ => println!("cargo::warning=\'Unexpeted env_key: `{}`\'", env_key),
    }
}
