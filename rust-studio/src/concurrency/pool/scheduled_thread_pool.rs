use std::time::Duration;
use std::sync::mpsc::channel;
pub fn scheduled_thread_pool() {
    let (sender, receiver) = channel();

    let pool = scheduled_thread_pool::ScheduledThreadPool::new(4);
    let handle = pool.execute_after(Duration::from_millis(1000), move || {
        println!("Hello from a scheduled thread!");
        sender.send("done").unwrap();
    });

    let _ = handle;
    receiver.recv().unwrap();
}
fn main(){
scheduled_thread_pool();
}