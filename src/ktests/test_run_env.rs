#[cfg(ktest_item = "run_env")]
include!(concat!(env!("OUT_DIR"), "/include.rs"));

#[cfg(ktest_item = "run_env")]
pub fn test_run_env() {
    use crate::kern::sched::schedule;

    setup_env_run();

    unsafe {
        schedule(false);
    }
}
