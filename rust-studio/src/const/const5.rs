#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
struct Item<const U: u8>;
#[derive(Debug)]
struct A;
#[derive(Debug)]
struct B;
#[derive(Debug)]
struct C;

impl Item<0> {
    fn foo() -> A {
        A
    }
}
impl Item<1> {
    fn foo() -> B {
        B
    }
}
impl Item<2> {
    fn foo() -> C {
        C
    }
}

const fn check(i: i32) -> u8 {
    match i {
        0 => 0,
        1.. => 1,
        _ => 2,
    }
}

// struct Foo<const I: i32 = 0> {}

// impl<const I: i32> Foo<I> where [(); (I < 0) as usize - 1]:,
// { fn f() {} }

// impl<const I: i32> Foo<I> where [(); (I > 0) as usize - 1]:,
// { fn f() {} }

// impl Foo<0>
// { fn f() {} }

// trait Greter{
// }
// trait Less{
// }
// trait Equal{
// }
// // error[E0592]: duplicate definitions with name `f`
// #![feature(generic_const_exprs)]
// #![allow(incomplete_features)]
// struct Check<const C: bool>;
// struct Foo<const I: i32> {}

// impl<const I: i32> Foo<I> where Check<{ check(I) }>: Greter,
// { fn f() {} }

// impl<const I: i32> Foo<I> where Check<{ check(I) }>: Less,
// { fn f() {} }

// impl<const I: i32> Foo<I> where Check<{ check(I) }>: Equal,
// { fn f() {} }

fn main() {
    dbg!(
        Item::<{ check(0) }>::foo(), // A
        Item::<{ check(1) }>::foo(), // B
        Item::<{ check(-1) }>::foo()
    ); // C
}
