// It shows how you can create a Future that uses a Rc to share data concurrently, running on a single thread.
// This is why the data is !Send, and we don’t need to use an Arc or Arc<Mutex> to share it between tasks.
// Once the LocalSet is created, and local_spawn() is called, 
// the task doesn’t actually run until local_set.run_until(..) is called, 
// or local_set.await is called.

use crossterm::style::Stylize;
use std::rc::Rc;
use tokio::{task::LocalSet, time::sleep};

#[tokio::test]
async fn run_local_set_and_spawn_local() {
    // Can't send this data across threads (not wrapped in `Arc` or `Arc<Mutex>`).
    let non_send_data = Rc::new("!SEND DATA");
    let local_set = LocalSet::new();

    // Spawn a local task (bound to same thread) that uses the non-send data.
    let non_send_data_clone = non_send_data.clone();

    let async_block_1 = async move {
        println!(
            // https://doc.rust-lang.org/std/fmt/index.html#fillalignment
            "{:<7} {}",
            "start",
            non_send_data_clone.as_ref().yellow().bold(),
        );
    };
    // Does not run anything.
    let join_handle_1 = local_set.spawn_local(async_block_1);

    // This is required to run `async_block_1`.
    let _it = local_set.run_until(join_handle_1).await;

//  This is just a different variant (from the first example) of creating a new async block, 
// and running it using the LocalSet.
 // Create a 2nd async block.
    let non_send_data_clone = non_send_data.clone();
    let async_block_2 = async move {
        sleep(std::time::Duration::from_millis(100)).await;
        println!(
            // https://doc.rust-lang.org/std/fmt/index.html#fillalignment
            "{:<7} {}",
            "middle",
            non_send_data_clone.as_ref().green().bold()
        );
    };

    // This is required to run `async_block_2`.
    let _it = local_set.run_until(async_block_2).await;


// This yet another way of how you can create a new async block, 
// and run it using the LocalSet. 
// This one uses local_set.await which runs all the futures that are associated with the local_set.

// Spawn another local task (bound to same thread) that uses
    // the non-send data.
    let non_send_data_clone = non_send_data.clone();
    let async_block_3 = async move {
        sleep(std::time::Duration::from_millis(100)).await;
        println!(
            // https://doc.rust-lang.org/std/fmt/index.html#fillalignment
            "{:<7} {}",
            "end",
            non_send_data_clone.as_ref().cyan().bold()
        );
    };
    // Does not run anything.
    let _join_handle_3 = local_set.spawn_local(async_block_3);

    // `async_block_3` won't run until this is called.
    local_set.await;
}