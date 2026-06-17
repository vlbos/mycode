 pub(crate) fn main() {
        use crossbeam_channel::bounded;

        let (s, r) = bounded(5);

        for i in 0..5 {
            s.send(i).unwrap();
        }
    }