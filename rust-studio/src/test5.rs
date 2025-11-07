use rand::{thread_rng, Rng};
fn main() {
    fn foo1<const N1: usize>(input: usize) { // 在泛型函数内，泛型常量参数的形参可用于
        let sum = 1 + N1 * input;   // #1 运行时求值的表达式
        let foo = Foo([input; N1]); // #5 结构体字段的值
        let arr: [usize; N1] = [input; N1]; // #4 绑定变量的数据类型 —— 编译时参数化数组长度
                                            // #5 绑定变量的值
        println!("运行时表达式：{sum},\n\
                  元组结构体：  {foo:?},\n\
                  数组：       {arr:?}");
    }
    trait Trait<const N2: usize> {
        const CONST: usize = N2 + 4; // #2 关联常量 + 常量表达式
        type Output;
    }
    #[derive(Debug)]
    struct Foo<const N3: usize>(
        [usize; N3] // #4 结构体字段的数据类型 —— 编译时参数化数组长度
    );
    impl<const N4: usize> Trait<N4> for Foo<N4> {
        type Output = [usize; N4]; // #3 关联类型 —— 编译时参数化数组长度
    }
    let mut rng = thread_rng();
    foo1::<2>(rng.gen_range::<usize, _>(1..10));
    foo1::<{1 + 2}>(rng.gen_range::<usize, _>(1..10));
    const K: usize = 3;
    foo1::<K>(rng.gen_range::<usize, _>(1..10));
    foo1::<{K * 2}>(rng.gen_range::<usize, _>(1..10));
}
