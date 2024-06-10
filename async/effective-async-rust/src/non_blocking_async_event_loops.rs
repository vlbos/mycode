#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn test_main_loop() {
    // Register tracing subscriber.
    tracing_subscriber::fmt()
        .without_time()
        .compact()
        .with_target(false)
        .with_line_number(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    // Create channels for events and shutdown signals.
    let event_channel = tokio::sync::mpsc::channel::<String>(1_000);
    let (event_sender, mut event_receiver) = event_channel;

    let shutdown_channel = tokio::sync::broadcast::channel::<()>(1_000);
    let (shutdown_sender, _) = shutdown_channel;

    // Spawn the main event loop.
    let mut shutdown_receiver = shutdown_sender.subscribe();
    let safe_count: std::sync::Arc<std::sync::Mutex<usize>> = Default::default();
    let safe_count_clone = safe_count.clone();
    let join_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                event = event_receiver.recv() => {
                    tracing::info!(?event, "task got event: event");
                    let mut count = safe_count_clone.lock().unwrap();
                    *count += 1;
                }
                _ = shutdown_receiver.recv() => {
                    tracing::info!("task got shutdown signal");
                    break;
                }
            }
        }
    });

    // Send events, in parallel.
    let mut handles = vec![];
    for i in 0..10 {
        let event_sender_clone = event_sender.clone();
        let join_handle = tokio::spawn(async move {
            tracing::info!(i, "sending event");
            let event = format!("event {}", i);
            let _ = event_sender_clone.send(event).await;
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        });
        handles.push(join_handle);
    }

    // Wait for all events to be sent using tokio.
    futures::future::join_all(handles).await;

    // Shutdown the event loops.
    shutdown_sender.send(()).unwrap();

    // Wait for the event loop to shutdown.
    join_handle.await.unwrap();

    // Assertions.
    assert_eq!(shutdown_sender.receiver_count(), 1);
    assert_eq!(*safe_count.lock().unwrap(), 10);
}

// We use tokio::sync::mpsc::channel to create a channel for events, 
// and tokio::sync::broadcast::channel to create a channel for shutdown signals.
// We spawn the main event loop, which listens for events and shutdown signals, and updates a shared counter.
// We spawn multiple tasks that send events to the event channel, in parallel.
// The #[tokio::test(flavor = "multi_thread", worker_threads = 5)] attribute macro tells tokio to run the test on multiple threads (max of 5).
// You can see this in the output when you run the test. By configuring Tokio tracing subscriber,
//  we can see the thread IDs and names in the output (.with_thread_ids(true), .with_thread_names(true)).
// We wait for all events to be sent using futures::future::join_all(handles).await.
// We shutdown the event loop (using shutdown_sender.send(())), and wait for it to shutdown using join_handle.await..