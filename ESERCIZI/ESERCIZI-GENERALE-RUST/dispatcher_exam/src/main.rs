use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};


struct Subscription<Msg: Clone> {
    receiver: Receiver<Msg>
}

struct Dispatcher<Msg: Clone> {
    senders: Mutex<Vec<Sender<Msg>>>
}


impl<Msg: Clone> Dispatcher<Msg> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            senders: Mutex::new(Vec::new())
        })
    }

    fn subscribe(&self) -> Subscription<Msg> {
        let mut senders = self.senders.lock().unwrap();
        let (sx, rx) = channel();
        senders.push(sx);
        Subscription {receiver: rx}
    }

    fn dispatch(&self, message: Msg) {
        let mut senders = self.senders.lock().unwrap();
        for i in 0..senders.len() {
            match senders[i].send(message.clone()) {
                Ok(_) => {},
                Err(_) => {
                    senders.remove(i);
                }
            }
        }
    }
}

impl <Msg: Clone> Subscription<Msg> {
    fn read(&self) -> Option<Msg> {
        match self.receiver.recv() {
            Ok(message) => Some(message),
            Err(_) => None
        }
    }
}











fn main() {
    println!("Hello, world!");
}
