 use std::sync::Arc;
use futures::poll;
 use futures::pin_mut;
pub fn async_weighted_semaphore_example() {
    smol::block_on(async {
        let sem = async_weighted_semaphore::Semaphore::new(1);
        let a = Box::pin(sem.acquire(2));
        let b = Box::pin(sem.acquire(1));
        pin_mut!(a);
        pin_mut!(b);
        assert!(poll!(&mut a).is_pending());

        assert!(poll!(&mut b).is_pending());
        sem.release(1);
        assert!(poll!(&mut a).is_ready());
        assert!(poll!(&mut b).is_ready());
    });
}

pub fn async_lock_semaphore() {
    let s = Arc::new(async_lock::Semaphore::new(2));
    let _g1 = s.try_acquire_arc().unwrap();
    let g2 = s.try_acquire_arc().unwrap();
    assert!(s.try_acquire_arc().is_none());
    drop(g2);
    assert!(s.try_acquire_arc().is_some());
}
fn main(){
async_lock_semaphore();
async_weighted_semaphore_example();
}