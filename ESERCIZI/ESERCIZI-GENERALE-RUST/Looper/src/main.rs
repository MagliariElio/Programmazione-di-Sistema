use std::fmt::Debug;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

struct Looper<Message: Send + 'static> {
    sender: Mutex<Sender<Message>>,
}

impl<Message: Send + 'static> Looper<Message> {
    fn new<T, Q>(process: T, cleanup: Q) -> Self
    where
        T: Fn(Message) + Send + 'static,
        Q: FnOnce() + Send + 'static,
    {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            loop {
                match receiver.recv() {
                    Ok(msg) => process(msg),
                    Err(_) => break,
                }
            }
            cleanup();
        });
        Self {
            sender: Mutex::new(sender),
        }
    }

    fn send(&self, msg: Message) {
        let sender = self.sender.lock().unwrap();
        sender.send(msg).unwrap();
    }
}

impl<Message: Send + 'static> Drop for Looper<Message> {
    fn drop(&mut self) {
        let sender = self.sender.lock().unwrap();
        drop(sender);
    }
}

fn main() {
    let l = Arc::new(Looper::new(process, cleanup));

    let mut vec = vec![];
    for i in 0..5 {
        vec.push(thread::spawn({
            let l_cloned = l.clone();
            move || {
                println!("Sending message #{}", i);
                l_cloned.send(i);
            }
        }))
    }

    for v in vec {
        v.join().unwrap();
    }
}

fn process<Message: Debug>(msg: Message) {
    println!("Processing {:?}", msg);
}

fn cleanup() {
    println!("cleaning...");
}
