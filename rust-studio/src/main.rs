#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// trait Test{
// fn build(&self){
// println!("Test");}
// }

// trait SubTest:Test{
// fn build(&self){
//   Test::build(self);
// println!("Sub");
// }
// }

// // struct A<T>{
// // t:T,
// // }
// // struct B<T>{
// // t:T,
// // }
// struct C;
// impl Test for C{
// }
// impl SubTest for C{
//     // fn build(&self){
//     //     // Test::build(self);
//     //     println!("c");
//     // }
// }

// struct A{
// a:i32,
// b:i32,
// }
// impl A{
//     pub fn set_a(v:&mut i32,vv:&mut i32){
//         *v+=1;
//         *vv+=2;
//     }
//     pub fn aa(&mut self){
//         Self::set_a(&mut self.a,&mut self.b);
//     }
// }
pub trait A {
    fn a() -> i32 {
        1
    }
}
struct B;
struct C;
impl A for B {
    fn a() -> i32 {
        2
    }
}
impl A for C {
    fn a() -> i32 {
        3
    }
}

struct ABC(pub i32);
const AA:ABC=ABC(2);
const AA2:ABC=ABC(3);
const AA3:ABC=AA*AA2;
impl const std::ops::Mul for ABC{
    type Output=Self;
    fn mul(self,rhs:ABC)->Self{
       ABC( self.0*rhs.0)
    }
}
struct TestC<const N:usize>;
pub trait Config<const N:usize> {
    const N2:usize=N*2;
    // const TC:TestC<{N*2}>;
    fn a() -> i32 {
       0
    }
    fn test()->Self;
}
struct Test;
impl Config<1> for Test{
    // const TC:TestC<2>=TestC::<2>;
    fn a() -> i32 {
      1
    }
    fn test()->Self{
    Test}
}
impl Config<2> for Test{
//  const TC:TestC<4>=TestC::<4>;
 fn a() -> i32 {
       2
    }
 fn test()->Self{
    Test}
}
use tracing::{Level,span};
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};
fn main() {
//  tracing_subscriber::fmt::init();
// let _ = tracing_subscriber::fmt().with_test_writer().try_init();
    tracing_subscriber::fmt()
        // NEW: 记录 Span 的开启 (New) 和关闭 (Close) 事件
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .init();
    // let formatting_layer = fmt::layer().pretty().with_writer(std::io::stdout);

    // Registry::default()
    //     .with(ErrorLayer::default())
    //     .with(formatting_layer)
    //     .init();
    let span=span!(Level::INFO,"info");
    let _=span.enter();
    //    SubTest::build(&C);
    // let mut a=A{a:0,b:1};
    // a.aa();
    // let c = C;
    // let b = B;
    // println!("{},{}", B::a(), C::a());

    let _test=Test;
    println!("{},{}", <Test as Config<2>>::a(), <Test as Config<1>>::a());
}
