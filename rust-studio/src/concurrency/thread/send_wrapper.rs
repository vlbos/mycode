use send_wrapper::SendWrapper;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::thread;
// pub fn wrong_send() {
//     let counter = Rc::new(42);

//     let (sender, receiver) = channel();

//     let _t = thread::spawn(move || {
//         sender.send(counter).unwrap();
//     });

//     let value = receiver.recv().unwrap();

//     println!("received from the main thread: {}", value);
// }

pub fn send_wrapper() {
    let wrapped_value = SendWrapper::new(Rc::new(42));

    let (sender, receiver) = channel();

    let _t = thread::spawn(move || {
        sender.send(wrapped_value).unwrap();
    });

    let wrapped_value = receiver.recv().unwrap();

    let value = wrapped_value.deref();
    println!("received from the main thread: {}", value);
}
fn main() {
    // wrong_send();
    send_wrapper();
}
