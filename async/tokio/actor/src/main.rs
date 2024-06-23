// Rust code snippet
use tokio::signal;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{ Duration};

// Define the types of commands that can be sent to the actor
enum Command {
    Add {
        x: i32,
        y: i32,
        result_sender: oneshot::Sender<i32>,
    },
    Concatenate {
        x: String,
        y: String,
        result_sender: oneshot::Sender<Result<String, ()>>,
    },
}

// Define the actor struct
struct Actor {
    receiver: mpsc::Receiver<Command>,
    name: String,
}

impl Actor {
    // Process incoming commands asynchronously
    async fn process(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::Add { x, y, result_sender } => {
                    let _ = result_sender.send(x + y); // Send the result of addition
                }
                Command::Concatenate { x, y, result_sender } => {
                    let result = if x.is_ascii() && y.is_ascii() {
                        Ok(format!("{}{}", x, y)) // Concatenate strings if both are ASCII
                    } else {
                        Err(()) // Return error if either string is non-ASCII
                    };
                    let _ = result_sender.send(result); // Send the result of concatenation
                }
            }
        }
    }

    // Access the actor's name
    fn name(&self) -> &str {
        &self.name
    }
}

// Define the proxy struct for interacting with the actor
#[derive(Clone)]
struct Proxy {
    sender: mpsc::Sender<Command>,
}

impl Proxy {
    // Send an addition command to the actor and await the result
    async fn add(&self, x: i32, y: i32) -> Result<i32, tokio::sync::mpsc::error::SendError<Command>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(Command::Add { x, y, result_sender }).await?;
        Ok(result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor")))
    }

    // Send a concatenation command to the actor and await the result
    async fn concatenate(&self, x: String, y: String) -> Result<String, tokio::sync::mpsc::error::SendError<Command>> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(Command::Concatenate { x, y, result_sender }).await?;
        let result = result_receiver.await.unwrap_or_else(|_| panic!("Failed to receive result from actor"));
        match result {
            Ok(value) => Ok(value),
            Err(_) => panic!("Failed to concatenate"),
        }
    }
}

// Initialize the actor and its corresponding proxy
fn init_actor_proxy(name: String, size: usize) -> (Actor, Proxy) {
    let (sender, receiver) = mpsc::channel(size);
    let actor = Actor { receiver, name: name.clone() };
    let proxy = Proxy { sender };
    (actor, proxy)
}

#[tokio::main]
async fn main() {
    // Initialize the actor and its proxy
    let (actor, proxy) = init_actor_proxy("MyActor".into(), 128);
    println!("{}",actor.name());
    // Spawn the actor's processing task
    tokio::spawn(actor.process());

    // Send some requests and evaluate responses
    assert_eq!(proxy.add(1, 2).await.unwrap(), 3);
    assert_eq!(
        proxy.concatenate(String::from("foo"), String::from("bar")).await.unwrap(),
        String::from("foobar")
    );

    // Spawn a separate task to demonstrate concurrent message passing
    let proxy2 = proxy.clone();
    tokio::spawn(async move {
        for i in 0..5 {
            println!("cool {}", i);
            // Sending messages from the main task to task_one
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
        println!("proxy.add(10, 5).await.unwrap() {}", proxy2.add(10, 5).await.unwrap());
    });

    // Wait for Ctrl+C signal to gracefully exit
    let ctrl_c = signal::ctrl_c();
    println!("Press Ctrl+C to exit...");
    ctrl_c.await.expect("Ctrl+C signal failed");
    println!("Ctrl+C received. Exiting...");
}