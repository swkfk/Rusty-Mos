//! Do the schedule job.

use crate::kdef::env::EnvStatus;

use super::env::{env_run, CUR_ENV, ENV_SCHE_LIST};

/// Record the env's rest time-slice for scheduling.
static mut ENV_REST_COUNT: u32 = 0;

/// Schedule the envs. If `yield`, the current env will be moved to the tail
/// of the schedule list. Otherwise, the strategy will judge the rest time the
/// env will enjoy.
///
/// We use the priority to represent the time-slice count of a env. If the count
/// run out, the next env will be selected.
///
/// # Return
/// The function is a *no-return* function.
/// [env_run](rusty_mos::kern::env::env_run) will run the selected env.
///
/// # Panic
///
/// The list is empty when we should pick one env to run. Only in this
/// situation, a panic will be raised.
///
/// # Safety
/// Actually, the list and the current env pointer **SHALL** be valid.
#[no_mangle]
pub unsafe fn schedule(r#yield: bool) -> ! {
    let mut env = CUR_ENV;
    if r#yield
        || ENV_REST_COUNT == 0
        || env.is_null()
        || (*(*env).data).status != EnvStatus::Runnable
    {
        if !env.is_null() && (*(*env).data).status == EnvStatus::Runnable {
            ENV_SCHE_LIST.remove(env);
            ENV_SCHE_LIST.insert_tail(env);
        }
        if ENV_SCHE_LIST.empty() {
            panic!("Schedule queue is empty. Terminated!");
        }
        env = ENV_SCHE_LIST.head; // TODO: Add a method for the list
        ENV_REST_COUNT = (*(*env).data).priority;
    }

    ENV_REST_COUNT -= 1;
    env_run(env);
}
