
pub fn main() {
    use crossbeam_channel::bounded;

    let (s, _r) = bounded(5);

    for i in 0..5 {
        s.send(i).unwrap();
    }
}
