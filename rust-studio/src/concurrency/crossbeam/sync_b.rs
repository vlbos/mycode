pub(crate) fn main() {
    use crossbeam_channel::unbounded;
    let (s, r) = unbounded();
    for i in 0..1000 {
        s.send(i).unwrap();
    }
}
