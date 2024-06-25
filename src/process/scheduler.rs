//! Do the schedule job.

use crate::process::envs::{env_run, EnvStatus, CUR_ENV_IDX, ENVS_DATA, ENV_SCHE_LIST, NENV};
use core::sync::atomic::{AtomicU32, Ordering::SeqCst};

/// Record the env's rest time-slice for scheduling.
static ENV_REST_COUNT: AtomicU32 = AtomicU32::new(0);

/// Schedule the envs. If `yield`, the current env will be moved to the tail
/// of the schedule list. Otherwise, the strategy will judge the rest time the
/// env will enjoy.
///
/// We use the priority to represent the time-slice count of a env. If the count
/// run out, the next env will be selected.
///
/// # Return
/// The function is a *no-return* function. [env_run] will run the selected env.
///
/// # Panic
///
/// The list is empty when we should pick one env to run. Only in this
/// situation, a panic will be raised.
///
/// # Safety
/// Actually, the list and the current env pointer **SHALL** be valid.
#[no_mangle]
pub fn schedule(r#yield: bool) -> ! {
    let mut env = CUR_ENV_IDX.load(SeqCst);
    if r#yield
        || ENV_REST_COUNT.load(SeqCst) == 0
        || env == NENV
        || ENVS_DATA.borrow().0[env].status != EnvStatus::Runnable
    {
        if env != NENV && ENVS_DATA.borrow().0[env].status == EnvStatus::Runnable {
            ENV_SCHE_LIST.borrow_mut().remove(env);
            ENV_SCHE_LIST.borrow_mut().insert_tail(env);
        }
        if ENV_SCHE_LIST.borrow().empty() {
            panic!("Schedule queue is empty. Terminated!");
        }
        env = ENV_SCHE_LIST.borrow().peek_head().unwrap();
        ENV_REST_COUNT.store(ENVS_DATA.borrow().0[env].priority, SeqCst);
    }

    let _count = ENV_REST_COUNT.fetch_sub(1, SeqCst);

    env_run(env);
}
