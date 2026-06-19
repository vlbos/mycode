use async_timer::Timed;
use std::time::Duration;

async fn long_running_job() {
    println!("task start...");
}
#[tokio::main]
pub async fn main() {
    let job_future = long_running_job();

    let timed_job = unsafe { Timed::platform_new_unchecked(job_future, Duration::from_secs(1)) };

    match timed_job.await {
        Ok(_) => println!("task done！"),
        Err(expired) => {
            println!("err:task expired：{:?}", expired);
        }
    }
}
