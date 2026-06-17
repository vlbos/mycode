use std::time::Duration;
use std::time::Instant;
use process_lock::ProcessLock;
pub fn process_lock() {
    let lock = ProcessLock::new(String::from(".process_lock"), None);
    let start = Instant::now();
    loop {
        if lock.is_ok() {
            println!("lock success");
            break;
        }
 if start.elapsed() > Duration::from_millis(500) {
        println!("lock timeout");
        break;
    }
std::thread::sleep(Duration::from_millis(100));
    }
   
    
    std::thread::sleep(Duration::from_millis(500));
}

use named_lock::NamedLock;
use named_lock::Result;
fn main() -> Result<()> {
    process_lock();
    let lock = NamedLock::create("foobar")?;
    let _guard = lock.lock()?;
    // Do something...
    println!("=====named lock")
    Ok(())
}
