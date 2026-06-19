
pub fn main() {
    let mut value_a = None;
    let mut value_b = None;
    let mut value_c = None;
    rayon::scope(|s| {
        s.spawn(|s1| {
            // ˆ`s` `handle` `s1`
            value_a = Some(22);
            //`s`
            s1.spawn(|_| {
                value_b = Some(44);
            });
        });
        s.spawn(|_| {
            value_c = Some(66);
        });
    });
    assert_eq!(value_a, Some(22));
    assert_eq!(value_b, Some(44));
    assert_eq!(value_c, Some(66));
}
