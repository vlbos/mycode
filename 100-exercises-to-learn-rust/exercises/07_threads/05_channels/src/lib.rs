use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
pub mod data;
pub mod store;
use crate::store::TicketStore;
use crate::data::TicketDraft;
pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
        let mut tickerStore=TicketStore::new();
        loop{
            if let Ok(Command::Insert(ticket_draft))=receiver.recv(){
                tickerStore.add_ticket(ticket_draft);
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    }
