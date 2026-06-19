
pub fn main() {
    use crossbeam_channel::unbounded;
    let (s, _r) = unbounded();
    for i in 0..1000 {
        s.send(i).unwrap();
    }
}
