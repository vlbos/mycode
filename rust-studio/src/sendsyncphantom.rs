use std::marker::PhantomData;

type T = std::rc::Rc<u8>;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<PhantomData<fn(T) -> T>>();
    assert_sync::<PhantomData<fn(T) -> T>>();
}