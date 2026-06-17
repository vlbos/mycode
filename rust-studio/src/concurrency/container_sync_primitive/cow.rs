use std::borrow::Cow;
// use dyn_clone::clone;
fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1; // s1 s2
    s2.push_str(" world"); // s2 , s2

    let origin = "hello world";
    let mut cow = Cow::from(origin);
    assert_eq!(cow, "hello world");
    // Cow can be borrowed as a str
    let s: &str = &cow;
    assert_eq!(s, "hello world");
    assert_eq!(s.len(), cow.len());
    // Cow can be converted to a String
    let s: String = cow.clone().into();
    assert_eq!(s, "HELLO WORLD");

    // Cow can be borrowed as a mut str
    let s: &mut str = cow.to_mut();
    s.make_ascii_uppercase();
    assert_eq!(s, "HELLO WORLD");
    assert_eq!(origin, "hello world");
    beef_cow();
}
pub fn beef_cow() {
    let borrowed: beef::Cow<str> = beef::Cow::borrowed("Hello");
    let owned: beef::Cow<str> = beef::Cow::owned(String::from("World"));
    let _ = beef::Cow::from("Hello");
    assert_eq!(format!("{} {}!", borrowed, owned), "Hello World!",);
    const WORD: usize = size_of::<usize>();
    assert_eq!(size_of::<std::borrow::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);
}
