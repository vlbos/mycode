//`foo()` `Future<Output = u8>`

//`foo().await` `Future` `u8`
// async fn foo() -> u8 {
//     5
// }
// fn bar() -> impl Future<Output = u8> {
//     //`async` `Future<Output = u8>`
//     async {
//         let x: u8 = foo().await;
//         x + 5
//     }
// }

async fn foo() -> Result<u8, String> {
    Ok(1)
}
async fn bar() -> Result<u8, String> {
    Ok(1)
}

// pub pub fn main1() {
//     let fut = async {
//         foo().await?;
//         bar().await?;
//         Ok(())
//     };
// }

// error[E0282]: type annotations needed
// --> src/main.rs:14:9
// |
// 11 | let fut = async {
// | --- consider giving`fut` a type
// ...
// 14 | Ok(1)
// | ˆˆ cannot infer type for type parameter`E` declared on the enum`Resul
pub fn main() {
    let _fut = async {
        foo().await?;
        bar().await?;
        Ok::<(), String>(())
    };
}
// pub trait Future {
//     type Output;
//     // Required method
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }
