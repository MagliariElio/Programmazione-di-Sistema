
/*
Implementare una struttura dati concorrente chiamata BoundedBlockingQueue che rappresenti una coda limitata con
capacità fissa. La coda dovrebbe consentire a più thread di inserire e rimuovere elementi contemporaneamente,
garantendo la sicurezza dei thread e un comportamento adeguato di blocco.

La BoundedBlockingQueue dovrebbe avere i seguenti metodi:

    new(capacity: usize) -> Self: Crea una nuova istanza della coda con la capacità specificata.

    enqueue(&self, item: T) -> Result<(), T>: Inserisce un elemento nella coda. Se la coda è già piena, questo metodo
        dovrebbe bloccare il thread corrente fino a quando non viene liberato spazio o fino a quando non scade un timeout specificato.

    dequeue(&self) -> Option<T>: Rimuove un elemento dalla coda. Se la coda è vuota, questo metodo dovrebbe bloccare
        il thread corrente fino a quando non diventa disponibile un elemento o fino a quando non scade un timeout specificato.

    size(&self) -> usize: Restituisce il numero attuale di elementi nella coda.

Implementare la BoundedBlockingQueue in Rust, garantendo la sicurezza dei thread e l'utilizzo di meccanismi
di sincronizzazione adeguati per ottenere il comportamento desiderato.
*/

use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(3);

struct BoundedBlockingQueue<T: Ord + PartialEq> {
    queue: Mutex<BinaryHeap<T>>,
    capacity: usize,
    cvar: Condvar
}

impl <T: Ord + PartialEq> BoundedBlockingQueue<T> {
    fn new(capacity: usize) -> Self {
        Self {
            queue: Mutex::new(BinaryHeap::with_capacity(capacity)),
            capacity,
            cvar: Condvar::new()
        }
    }

    fn enqueue(&self, item: T) -> Result<(), T> {
        let mut queue = self.queue.lock().unwrap();
        let timeout_current = Instant::now() + TIMEOUT;
        while self.capacity == queue.len() {
            if timeout_current < Instant::now() {
                return Err(item);
            }
            queue = self.cvar.wait_timeout(queue, TIMEOUT,).unwrap().0;
        }
        queue.push(item);
        self.cvar.notify_all();
        return Ok(());
    }

    fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue = self.cvar.wait_timeout_while(queue, TIMEOUT, |value| value.len() == 0).unwrap().0;
        let result = queue.pop();
        self.cvar.notify_all();
        result
    }

    fn size(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }

}

fn main() {
    let capacity = 5;
    let queue = Arc::new(BoundedBlockingQueue::new(capacity));

    let producer = {
        let queue = queue.clone();
        thread::spawn(move || {
            for i in 1..=10 {
                println!("Produce: {}", i);
                queue.enqueue(i).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        })
    };

    let consumer = {
        let queue = queue.clone();
        thread::spawn(move || {
            for _ in 1..=10 {
                let item = queue.dequeue().unwrap();
                println!("Consume: {} len: {}", item, queue.size());
                thread::sleep(Duration::from_millis(200));
            }
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
}