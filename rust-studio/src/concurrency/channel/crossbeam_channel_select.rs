pub(crate) fn main() {
    use crossbeam_channel::{select, unbounded};
    let (s1, r1) = unbounded();
    let (s2, r2) = unbounded();
    s1.send(10).unwrap();
    // Since both operations are initially ready, a random one will be executed.
    select! {
    recv(r1) -> msg => assert_eq!(msg, Ok(10)),
    send(s2, 20) -> res => {
    assert_eq!(res, Ok(()));
    assert_eq!(r2.recv(), Ok(20));
    }
    default => println!("not ready"),
    }
}
