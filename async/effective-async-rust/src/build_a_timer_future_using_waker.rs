use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use crossterm::style::Stylize;
// Build a timer that wakes up a task after a certain amount of time, 
// to explore how Waker works.
// We’ll just spin up a new thread when the timer is created,
//  sleep for the required time, 
// and then signal the timer future when the time window has elapsed.

// Add the following code to the file, 
// to define a new struct that will implement the Future trait. 
// This struct will have a SharedState struct that will contain the state of the future, 
// and an optional Waker that will be used to wake up the future when the timer has elapsed. 
// This Waker is not available until the very first time the future is polled by the runtime.
#[derive(Default)]
pub struct TimerFuture {
    pub shared_state: Arc<Mutex<SharedState>>,
}

#[derive(Default)]
pub struct SharedState {
    pub completed: bool,
    pub waker: Option<Waker>,
}

// Add the following code to implement the Future trait for the TimerFuture struct.

// This code will be used to poll the future, by the runtime, and check if the timer has elapsed.
// If it has, then the future is complete, and the runtime can move on to the next task. 
// If the timer has not elapsed, then the future is not complete, 
// and the runtime won’t do anything further with this future.
//  And will go on to the next task (top level Future) that it can make progress on.
// Something has to wake up this future to let the runtime know that the timer has elapsed,
//  and that it needs to call poll() again on this Future. This is where the Waker comes in.

// The first time poll() is called on this future, the runtime passes in a Waker and we save that to the SharedState struct.
// This will be used by the timer thread to wake up the future, when the timer has elapsed (which we will do next).

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        match shared_state.completed {
            true => {
                eprintln!("{}", "TimerFuture is completed".to_string().green());
                Poll::Ready(())
            }
            false => {
                eprintln!("{}", "TimerFuture is not completed".to_string().red());
                // Importantly, we have to update the Waker every time the
                // future is polled because the future may have moved to
                // a different task with a different Waker. This will happen
                // when futures are passed around between tasks after being
                // polled.
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

// Add the following code to create a new timer Future, 
// and start a new thread that will sleep for the required time, 
// and then wake up the Future when the timer has elapsed,
//  by using the optional Waker that was saved in the SharedState struct (when poll() is called on the Future,
//  by the runtime).

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let new_instance = TimerFuture::default();

        let shared_state_clone = new_instance.shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = shared_state_clone.lock().unwrap();
            shared_state.completed = true;
            shared_state.waker.take().unwrap().wake();
        });

        new_instance
    }
}

// Add the following test to run this code. 
// The #[tokio::test] attribute macro generates code to start a single threaded executor to run the test code.
#[tokio::test]
async fn run_timer_future_with_tokio() {
    let timer_future = TimerFuture::new(Duration::from_millis(10));
    let shared_state = timer_future.shared_state.clone();
    assert!(!shared_state.lock().unwrap().completed);
    timer_future.await;
    assert!(shared_state.lock().unwrap().completed);
}