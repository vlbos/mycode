// This code shows how you can use async_stream crate’s stream! macro to create a stream of values that are generated from a vector of strings.
// This stream is then converted into a PinnedInputStream which is a Pin<Box<dyn Stream<Item = Result<String, String>>>.

use futures::Stream;
use futures::StreamExt;
use std::pin::Pin;
pub type PinnedInputStream = Pin<Box<dyn Stream<Item = Result<String, String>>>>;

pub fn gen_input_stream() -> PinnedInputStream {
    let it = async_stream::stream! {
        for event in get_input_vec() {
            yield Ok(event);
        }
    };
    Box::pin(it)
}

pub fn get_input_vec() -> Vec<String> {
    vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ]
}

#[tokio::test]
async fn test_stream() {
    let mut count = 0;
    let mut it = gen_input_stream();
    while let Some(event) = it.next().await {
        let lhs = event.unwrap();
        let rhs = get_input_vec()[count].clone();
        assert_eq!(lhs, rhs);
        count += 1;
    }
}