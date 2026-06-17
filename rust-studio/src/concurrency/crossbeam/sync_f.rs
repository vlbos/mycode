pub(crate) fn main() {
    use crossbeam_channel::unbounded;
    use std::thread;
    let (s, r) = unbounded();
    thread::spawn(move || {
        s.send(1).unwrap();
        s.send(2).unwrap();
        s.send(3).unwrap();
        drop(s);
    });
    //`collect`
    let v: Vec<_> = r.iter().collect();
    assert_eq!(v, [1, 2, 3]);
}
