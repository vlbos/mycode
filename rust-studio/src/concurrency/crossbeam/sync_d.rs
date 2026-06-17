 pub(crate) fn main() {
        use crossbeam_channel::bounded;
        use std::thread;
        let (s1, r1) = bounded(0);
        let (s2, r2) = (s1.clone(), r1.clone());
        // ,
        thread::spawn(move || {
            r2.recv().unwrap();
            s2.send(2).unwrap();
        });
        s1.send(1).unwrap();
        r1.recv().unwrap();
    }