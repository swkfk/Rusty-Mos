use crate::kdef::env::EnvStatus;

use super::env::{env_run, CUR_ENV, ENV_SCHE_LIST};

static mut ENV_REST_COUNT: u32 = 0;

/// # Safety
///
pub unsafe fn sched(r#yield: bool) -> ! {
    let mut env = CUR_ENV;
    if r#yield || ENV_REST_COUNT == 0 || env.is_null() || (*env).data.status != EnvStatus::Runnable
    {
        if !env.is_null() && (*env).data.status == EnvStatus::Runnable {
            ENV_SCHE_LIST.remove(env);
            ENV_SCHE_LIST.insert_tail(env);
        }
        if ENV_SCHE_LIST.empty() {
            panic!("Schedule queue is empty. Terminated!");
        }
        env = ENV_SCHE_LIST.head; // TODO: Add a method for the list
        ENV_REST_COUNT = (*env).data.priority;
    }

    ENV_REST_COUNT -= 1;
    env_run(env);
}
