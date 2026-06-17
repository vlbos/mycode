trait Config {
    type T;
}
trait Config2: Config {}
trait Config3 {
    type C: Config2<T = i32>;
}
trait Config4<T> {
    fn test(&self, t: T);
}
trait Config6{
    const M:usize=2;
}
struct B<const M:usize>{
    t:[i32;M],
}
struct A;
impl Config4<i32> for A {
    fn test(&self, t: i32) {
        println!("i32==t");
    }
}
impl Config4<i8> for A {
    fn test(&self, t: i8) {
        println!("i8==t");
    }
}
trait Config5 {
    fn tests<T>(&self) -> Vec<T>;
}
impl Config5 for A {
     fn tests<A>(&self) -> Vec<A> {
        vec![]
    }
}

fn main() {
    let a = A;
    a.test(0i8);
    a.test(0);
}
