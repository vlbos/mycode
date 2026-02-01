#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Item<const I: i32> {}

#[allow(dead_code)]
struct Assert<const I: i32>;

#[allow(dead_code)]
impl<const I: i32> Assert<I> {
    const OK: usize = {
        assert!(I != 0);
        0
    };
}

impl<const I: i32> Item<I> where [(); Assert::<{ I }>::OK]:
{
    fn for_non_zero() {}
}

fn main() {
    Item::<1>::for_non_zero();
    // 下面这一行代码导致编译错误
    Item::<0>::for_non_zero();
}