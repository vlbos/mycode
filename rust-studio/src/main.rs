trait Test{
fn build(&self){
println!("Test");}
}

trait SubTest:Test{
fn build(&self){
  Test::build(self);
println!("Sub");
}
}

// struct A<T>{
// t:T,
// }
// struct B<T>{
// t:T,
// }
struct C;
impl Test for C{
}
impl SubTest for C{
    // fn build(&self){
    //     // Test::build(self);
    //     println!("c");
    // }
}

struct A{
a:i32,
b:i32,
}
impl A{
    pub fn set_a(v:&mut i32,vv:&mut i32){
        *v+=1;
        *vv+=2;
    }
    pub fn aa(&mut self){
        Self::set_a(&mut self.a,&mut self.b);
    }
}
fn main() {
//    SubTest::build(&C);
    let mut a=A{a:0,b:1};
    a.aa();
}



