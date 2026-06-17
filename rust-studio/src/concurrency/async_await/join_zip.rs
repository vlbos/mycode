use anyhow::{Error, anyhow};
fn main() {
    use futures::future::{FutureExt, select};
    use futures::pin_mut;
    let future1 = async { /* future 1 */ };
    let future2 = async { /* future 2 */ };

    let result = async {
        let future1 = future1.fuse();
        let future2 = future2.fuse();
        pin_mut!(future1, future2);
        futures::select! {
        res1 = future1 => { /* handle result of future1 */ },
        res2 = future2 => { /* handle result of future2 */ },
        }
    };

    use futures::future::join;
    let future1 = async { /* future 1 */ };
    let future2 = async { /* future 2 */ };
    let (res1, res2) = futures::executor::block_on(async { futures::join!(future1, future2) });

    use futures::try_join;

    let future1 = async { Ok::<(), Error>(()) };
    let future2 = async { Err(anyhow!("SomethingBad")) };
    async {
        let result: Result<((), ()), Error> = try_join!(future1, future2);
    };
}
pub fn smol_zip() {
    smol::block_on(async {
        use smol::future::{FutureExt, try_zip, zip};
        let future1 = async { 1 };
        let future2 = async { 2 };
        let result = zip(future1, future2);
        println!("smol_zip: {:?}", result.await);
        let future1 = async { Ok::<i32, i32>(1) };
        let future2 = async { Err::<i32, i32>(2) };
        let result = try_zip(future1, future2).await;
        println!("smol_try_zip: {:?}", result);
    });
}
