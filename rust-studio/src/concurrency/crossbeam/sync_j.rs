pub(crate) fn main() {
    use crossbeam_utils::sync::Parker;
    use std::thread;
    use std::time::Duration;
    let p = Parker::new();
    let u = p.unparker().clone();
    u.unpark();
    p.park();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        u.unpark();
    });
    //`u.unpark()`
    p.park();
}
