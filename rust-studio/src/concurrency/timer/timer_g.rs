use std::time::Duration;
use timer_kit::sleep;

async fn generic_delayed_task<D>()
where
    D: timer_kit::Delay,
    <D as timer_kit::Delay>::Instant: Unpin,
{
    println!("start...");

    sleep::<D>(Duration::from_secs(2)).await;

    println!("done");
}

pub async fn main() {
    println!("---  Tokio ");

    generic_delayed_task::<tokio::time::Sleep>().await;
}
