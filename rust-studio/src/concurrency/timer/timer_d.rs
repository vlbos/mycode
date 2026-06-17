use async_io::Timer;
    use std::time::Duration;
    use futures_lite::prelude::*;
     pub(crate) async fn main() {
        Timer::after(Duration::from_secs(1)).await;

        // use async_io::Timer;
        // use futures_lite::prelude::*;
        // use std::time::Duration;
        async fn run_with_timeout(timeout: Option<Duration>) {
            let timer = timeout
                .map(|timeout| Timer::after(timeout))
                .unwrap_or_else(Timer::never);
            // run_lengthy_operation().or(timer).await;
            timer.await;
        }
        // Times out after 5 seconds.
        run_with_timeout(Some(Duration::from_secs(5))).await;
        tokio::spawn(async move {
            // Does not time out.
            run_with_timeout(None).await;
        });
        

        // use async_io::Timer;
        // use std::time::Duration;
        Timer::after(Duration::from_secs(1)).await;

        // use async_io::Timer;
        use std::time::{ Instant};
        let now = Instant::now();
        let when = now + Duration::from_secs(1);
        Timer::at(when).await;

        // use async_io::Timer;
        // use futures_lite::StreamExt;
        // use std::time::{Duration, Instant};
        let period = Duration::from_secs(1);
        Timer::interval(period).next().await;

        // use async_io::Timer;
        // use futures_lite::StreamExt;
        // use std::time::{Duration, Instant};
        let start = Instant::now();
        let period = Duration::from_secs(1);
        Timer::interval_at(start, period).next().await;
    }