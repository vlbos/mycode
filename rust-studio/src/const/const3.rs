#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(negative_impls)]

struct Item<const I: i32>;

impl<const I: i32> Item<I>
where
    Check<{ I != 0 }>: NonZero,
{
    fn for_non_zero() {}
}

struct Check<const C: bool>;
trait NonZero {}
impl NonZero for Check<true> {}
impl !NonZero for Check<false> {} // 这一步在这里并不是必要的

fn main() {
    Item::<1>::for_non_zero();
    // Error:
    Item::<0>::for_non_zero();
}