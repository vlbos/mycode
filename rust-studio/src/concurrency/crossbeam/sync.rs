fn main() {
    use crossbeam_utils::sync::WaitGroup;
    use std::thread;
    let wg = WaitGroup::new();
    for _ in 0..4 {
        let wg = wg.clone();
        thread::spawn(move || {
            drop(wg);
        });
    }
    wg.wait();
}
