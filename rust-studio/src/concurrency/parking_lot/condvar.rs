fn main() {
    use parking_lot::{Condvar, Mutex};
    use std::sync::Arc;
    use std::thread;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Inside of our lock, spawn a new thread, and then wait for it to start
    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock();
        *started = true;
        cvar.notify_one();
    });

    // wait for the thread to start up
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock();
    if !*started {
        cvar.wait(&mut started);
    }
    // Note that we used an if instead of a while loop above. This is only
    // possible because parking_lot's Condvar will never spuriously wake up.
    // This means that wait() will only return after notify_one or notify_all is
    // called.
}
