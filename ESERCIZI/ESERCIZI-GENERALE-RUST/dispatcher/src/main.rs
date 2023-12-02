use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

struct Dispatcher<Msg: Clone + 'static> {
    senders: Mutex<Vec<Sender<Msg>>>,
}

struct Subscription<Msg: Clone + 'static> {
    receiver: Receiver<Msg>,
}

impl<Msg: Clone + 'static> Dispatcher<Msg> {
    fn new() -> Self {
        Self {
            senders: Mutex::new(Vec::new()),
        }
    }

    fn dispatch(&self, msg: Msg) {
        let senders = self.senders.lock().unwrap();
        for sender in senders.iter() {
            let _ = sender.send(msg.clone());
        }
    }

    fn subscribe(&self) -> Subscription<Msg> {
        let mut senders = self.senders.lock().unwrap();
        let (sender, receiver) = channel::<Msg>();
        senders.push(sender);
        return Subscription { receiver };
    }
}

impl<Msg: Clone + 'static> Drop for Dispatcher<Msg> {
    fn drop(&mut self) {}
}

impl<Msg: Clone + 'static> Subscription<Msg> {
    fn read(&self) -> Result<Msg, RecvError> {
        match self.receiver.recv() {
            Ok(msg) => Ok(msg),
            Err(m) => Err(m)
        }
    }
}

impl<Msg: Clone + 'static> Drop for Subscription<Msg> {
    fn drop(&mut self) {}
}

fn main() {
    let dispatcher = Arc::new(Dispatcher::new());

    let mut handles = vec![];

    for i in 0..10 {
        handles.push(thread::spawn({
            //clono il riferimento al dispatcher in modo da poterlo chiamare da più threads
            let d = dispatcher.clone();
            move || {
                let time = rand::thread_rng().gen_range(0..5);
                sleep(Duration::from_secs(time));
                let sub = d.subscribe();
                //il dispatcher è multiple-producer e può essere utilizzato da più threads insieme
                d.dispatch("from thread ".to_string() + i.to_string().as_str());
                //è ESSENZIALE effettuare la drop del riferimento prima di fare la read()
                // infatti dispatcher rimane in vita finché esiste almeno un riferimento ad esso
                // e se il thread possiede un riferimento mentre fa la read richia di mandarsi da solo in deadlock
                std::mem::drop(d);
                loop {
                    let time = rand::thread_rng().gen_range(10..100);
                    sleep(Duration::from_millis(time)); //helps print to remain mostly in-order
                    let res = sub.read();
                    match res {
                        Err(_) => {
                            println!("Thread {i} returns DUE TO THE DISPATCHER");
                            break;
                        }
                        Ok(msg) => {
                            println!("    thread {} received msg {} ", i, msg)
                        }
                    }
                    let early_drop = rand::thread_rng().gen_range(0..10);
                    if early_drop == 0 {
                        println!("Thread {i} returns EARLY");
                        drop(sub);
                        break;
                    }
                }
            }
        }))
    }

    for i in 30..35 {
        println!("> Dispatching value {i}");
        let time = rand::thread_rng().gen_range(2..4);
        sleep(Duration::from_secs(time));
        dispatcher.dispatch(i.to_string() + " from main");
    }

    //il main possiede un riferimento al dispatcher che deve essere eliminato (insieme a tutti gli altri)
    // per poter permettere alle read() in attesa di ritornare, una volta che non si vogliono più inviare messaggi
    drop(dispatcher);

    for h in handles {
        h.join().unwrap();
    }
}
