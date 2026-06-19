
pub fn main() {
    use crossbeam_channel::bounded;
    use std::thread;

    let (s, r) = bounded(0);
    thread::spawn(move || s.send("Hi!").unwrap());
    assert_eq!(r.recv(), Ok("Hi!"));
}
