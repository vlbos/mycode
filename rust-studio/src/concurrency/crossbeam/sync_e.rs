 pub(crate) fn main() {
        use crossbeam_channel::{unbounded, RecvError};
        let (s, r) = unbounded();
        s.send(1).unwrap();
        s.send(2).unwrap();
        s.send(3).unwrap();
        drop(s);
        assert_eq!(r.recv(), Ok(1));
        assert_eq!(r.recv(), Ok(2));
        assert_eq!(r.recv(), Ok(3));
        assert!(r.is_empty());
        //`r.recv()`
        //`Err(RecvError)`
        assert_eq!(r.recv(), Err(RecvError));
    }