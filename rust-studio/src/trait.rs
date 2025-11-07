#![allow(incomplete_features, dead_code)]
#![feature(generic_const_exprs)]
trait Foo {
    const N: usize;
    fn foo() -> [u8; Self::N];
}

#![allow(incomplete_features, dead_code, non_upper_case_globals)]
#![feature(generic_const_exprs)]
struct Point<const NDims: usize>([f64; NDims]);
pub trait Config {
    const NDims: usize;
}
struct ColorSpace<C: Config> where [(); C::NDims]: {
    data: Vec<(Point<{C::NDims}>, u8)>,
}

#![allow(incomplete_features, dead_code)]
#![feature(generic_const_exprs, generic_const_items)]
pub trait Foo {
    const COUNT: usize;
    const NAMES: [&'static str; Self::COUNT] where [(); Self::COUNT]:;
}

pub struct Bar<T: Foo> where [(); T::COUNT]: {
    name_lookup: [String; T::COUNT]
}


#![allow(incomplete_features, dead_code)]
#![feature(generic_const_exprs)]
trait VectorSpaceBase {
    type Scalar;
    const D: usize;
}

trait VectorSpace: VectorSpaceBase 
    + Into<[Self::Scalar; Self::D]> 
    + From<[Self::Scalar; Self::D]>
where 
    [(); Self::D]:,
{}