#![feature(thin_box)]
use std::r#box::ThinBox;
fn main1() {
    let val: u8 = 5;
    let boxed: Box<u8> = Box::new(val);

    let boxed: Box<u8> = Box::new(5);
    let val: u8 = *boxed;
}
// #[derive(Debug)]
// enum List<T> {
//     Cons(T, List<T>),
//     Nil,
// }

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
fn main() {
    main1();
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
    thin_box_example();
}

pub fn thin_box_example() {
    use std::mem::{size_of, size_of_val};
    let size_of_ptr = size_of::<*const ()>();
    let box_five = Box::new(5);
    let box_slice = Box::<[i32]>::new_zeroed_slice(5);
    assert_eq!(size_of_ptr, size_of_val(&box_five));
    assert_eq!(size_of_ptr * 2, size_of_val(&box_slice));
    let five = ThinBox::new(5);
    let thin_slice = ThinBox::<[i32]>::new_unsize([1, 2, 3, 4]);
    assert_eq!(size_of_ptr, size_of_val(&five));
    assert_eq!(size_of_ptr, size_of_val(&thin_slice));
}
