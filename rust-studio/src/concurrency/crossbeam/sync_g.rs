
pub fn main() {
    use crossbeam_channel::{select, unbounded};
    use std::thread;
    use std::time::Duration;
    let (s1, r1) = unbounded();
    let (s2, r2) = unbounded();
    thread::spawn(move || s1.send(10).unwrap());
    thread::spawn(move || s2.send(20).unwrap());
    // recv
    select! {
    recv(r1) -> msg => assert_eq!(msg, Ok(10)),
    recv(r2) -> msg => assert_eq!(msg, Ok(20)),
    default(Duration::from_secs(1)) => println!("timed out"),
    }
}
