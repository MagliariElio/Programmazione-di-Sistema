/*
    Una DelayedQueue<T:Send> è un particolare tipo di coda non limitata che offre tre metodi
    principali, oltre alla funzione costruttrice:

    1. offer(&self, t:T, i: Instant) : Inserisce un elemento che non potrà essere estratto prima
        dell'istante di scadenza i.
    2. take(&self) -> Option<T>: Cerca l'elemento t con scadenza più ravvicinata: se tale
        scadenza è già stata oltrepassata, restituisce Some(t); se la scadenza non è ancora stata
        superata, attende senza consumare cicli di CPU, che tale tempo trascorra, per poi restituire
        Some(t); se non è presente nessun elemento in coda, restituisce None. Se, durante l'attesa,
        avviene un cambiamento qualsiasi al contenuto della coda, ripete il procedimento suddetto
        con il nuovo elemento a scadenza più ravvicinata (ammesso che ci sia ancora).
    3. size(&self) -> usize: restituisce il numero di elementi in coda indipendentemente dal fatto
        che siano scaduti o meno.

    Si implementi tale struttura dati nel linguaggio Rust, avendo cura di renderne il comportamento
    thread-safe. Si ricordi che gli oggetti di tipo Condvar offrono un meccanismo di attesa limitata nel
    tempo, offerto dai metodi wait_timeout(...) e wait_timeout_while(...)).
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Element<T: Send + Ord> {
    timeout: Instant,
    value: T,
}

struct DelayedQueue<T: Send + Ord> {
    queue: Mutex<BinaryHeap<Reverse<Element<T>>>>,
    cvar: Condvar,
}

impl<T: Send + Ord> DelayedQueue<T> {
    fn new() -> Self {
        Self {
            queue: Mutex::new(BinaryHeap::new()),
            cvar: Condvar::new(),
        }
    }

    fn offer(&self, t: T, i: Instant) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(Reverse(Element {
            value: t,
            timeout: i,
        }));
        self.cvar.notify_all();
    }

    fn take(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() == 0 {
            return None;
        }

        loop {
            match queue.peek() {
                Some(element) => {
                    let length = queue.len();
                    let timeout = element.0.timeout;
                    let current = Instant::now();
                    if current > timeout {
                        queue = self
                            .cvar
                            .wait_timeout_while(queue, Duration::from(timeout - current), |value| {
                                value.len() == length
                            })
                            .unwrap()
                            .0;
                    }

                    if length == queue.len() && current >= timeout {
                        return Some(queue.pop().unwrap().0.value);
                    }
                }
                None => {} // None solo se non c'è nessun elemento
            }
        }
    }

    fn size(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }
}

fn main() {
    let delayed_queue = Arc::new(DelayedQueue::new());

    let producer = {
        let delayed_queue = Arc::clone(&delayed_queue);
        std::thread::spawn(move || {
            let instant = Instant::now() + Duration::from_secs(0);
            delayed_queue.offer(4, instant);
            println!("Offered item {} at {:?}", 0, instant);
            let instant = Instant::now() + Duration::from_secs(2);
            delayed_queue.offer(3, instant);
            println!("Offered item {} at {:?}", 2, instant);
            let instant = Instant::now() + Duration::from_secs(1);
            delayed_queue.offer(2, instant);
            println!("Offered item {} at {:?}", 1, instant);
            let instant = Instant::now() + Duration::from_secs(1);
            delayed_queue.offer(1, instant);
            println!("Offered item {} at {:?}", 1, instant);
        })
    };

    producer.join().unwrap();
    println!("{}", delayed_queue.size());

    let consumer = {
        let delayed_queue = Arc::clone(&delayed_queue);
        std::thread::spawn(move || {
            for _ in 1..=5 {
                if let Some(item) = delayed_queue.take() {
                    println!("Taken item: {:?}", item);
                    println!("Size: {:?}", delayed_queue.size());
                } else {
                    println!("None");
                }
            }
        })
    };

    consumer.join().unwrap();
}
