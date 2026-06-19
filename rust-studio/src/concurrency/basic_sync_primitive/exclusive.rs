// #![feature(exclusive_wrapper)]
// use std::sync::Exclusive;
//  pub fn main() {
//     let mut exclusive = Exclusive::new(92);
//     println!("ready");
//     std::thread::spawn(move || {
//         let counter = exclusive.get_mut();
//         println!("{}", *counter);
//         *counter = 100;
//     })
//     .join()
//     .unwrap();
// }
