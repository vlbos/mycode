#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Item<const I: i32> {}

impl<const I: i32> Item<I>
where
    [(); (I != 0) as usize - 1]:, // 这里的技巧在常量表达式中非常常见
{
    fn for_non_zero() {}
}

fn main() {
    Item::<1>::for_non_zero();
    // 下面这一行代码导致编译错误
    Item::<0>::for_non_zero();
}