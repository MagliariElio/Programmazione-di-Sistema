use std::sync::mpsc::{channel, Sender};
use std::{mem, thread};
use std::thread::JoinHandle;

struct Looper<Message>
    where
        Message: Send + 'static,
{
    sender: Sender<Message>
}

impl<Message> Looper<Message>
    where
        Message: Send + 'static,
{
    fn new<P, FC>(process: P, cleanup: FC) -> Self
        where
            P: Fn(Message) + Send + 'static,
            FC: FnOnce() + Send + 'static,
    {
        let (sx, rx) = channel();

        thread::spawn(move || loop {
            match rx.recv() {
                Ok(message) => {
                    process(message);
                }
                Err(_) => {
                    cleanup();
                    return;
                }
            }
        });

        Self { sender: sx }
    }

    pub fn send(&self, message: Message) {
        self.sender.send(message).unwrap();
    }
}

impl<Message> Drop for Looper<Message>
    where
        Message: Send + 'static,
{
    fn drop(&mut self) {
        let older_sender = mem::replace(&mut self.sender, channel().0);
        drop(older_sender);
    }
}