/*
Sistema di Code con Scadenza

Un sistema concorrente richiede la gestione di code speciali in cui ciascun elemento ha un tempo di scadenza associato.
Implementa una struttura dati chiamata TimedQueue<T: Send> per gestire queste code.

La TimedQueue dovrebbe avere i seguenti metodi:

    new() -> Self: Crea una nuova istanza della coda.

    enqueue(&self, item: T, deadline: Instant) -> Result<(), T>: Inserisce un elemento nella coda con un tempo di scadenza specificato.
        Se il tempo di scadenza è già passato, restituisci un errore con l'elemento. In caso contrario, inserisci l'elemento nella coda
        e restituisci un risultato di successo.

    dequeue(&self) -> Option<T>: Rimuove e restituisce un elemento dalla coda che ha raggiunto o superato il tempo di scadenza.
        Se non ci sono elementi scaduti, attendi senza consumare cicli di CPU fino a quando un elemento scade. Restituisci l'elemento
        scaduto o None se la coda è vuota e non ci sono elementi scaduti.

    size(&self) -> usize: Restituisce il numero attuale di elementi nella coda, indipendentemente dal fatto che siano scaduti o meno.

Implementa la TimedQueue in Rust, garantendo la sicurezza dei thread e utilizzando i meccanismi di sincronizzazione adeguati per
ottenere il comportamento desiderato. Assicurati che la tua implementazione sia thread-safe e gestisca correttamente la concorrenza
tra le operazioni di inserimento e rimozione degli elementi scaduti.
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Element<T: Send + Ord> {
    timeout: Instant,
    value: T,
}

struct TimedQueue<T: Send + Ord> {
    queue: Mutex<BinaryHeap<Reverse<Element<T>>>>,
    cvar: Condvar,
}

impl<T: Send + Ord> TimedQueue<T> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: Mutex::new(BinaryHeap::new()),
            cvar: Condvar::new(),
        })
    }

    fn enqueue(&self, item: T, deadline: Instant) -> Result<(), T> {
        if deadline < Instant::now() {
            return Err(item);
        }
        let mut queue = self.queue.lock().unwrap();
        queue.push(Reverse(Element {
            value: item,
            timeout: deadline,
        }));
        Ok(())
    }

    fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() == 0 {
            return None;
        }

        match queue.pop() {
            Some(element) => {
                while element.0.timeout < Instant::now() {
                    queue = self
                        .cvar
                        .wait_timeout(queue, Instant::now() - element.0.timeout)
                        .unwrap()
                        .0;
                }
                return Some(element.0.value);
            }
            None => None,
        }
    }

    fn size(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        return queue.len();
    }
}

fn main() {
    // Crea una coda condivisa
    let timed_queue = TimedQueue::new();

    // Flag per il thread produttore
    let producer_finished = Arc::new(AtomicBool::new(false));

    // Thread produttore
    let producer_timed_queue = Arc::clone(&timed_queue);
    let producer_finished_flag = Arc::clone(&producer_finished);

    let producer = thread::spawn(move || {
        for i in 1..=5 {
            let instant = Instant::now() + Duration::from_secs(i);
            producer_timed_queue.enqueue(i, instant).unwrap();
            println!("Enqueued item {} with deadline {:?}", i, instant);
            thread::sleep(Duration::from_secs(1));
        }

        println!("{}", producer_timed_queue.size());

        // Segnala che il produttore ha finito
        producer_finished_flag.store(true, Ordering::SeqCst);
    });

    producer.join().unwrap();

    // Thread consumatore
    let consumer_timed_queue = Arc::clone(&timed_queue);
    let consumer = thread::spawn(move || {
        loop {
            match consumer_timed_queue.dequeue() {
                Some(item) => println!("Dequeued item: {:?}", item),
                None => {
                    // Controlla se il produttore ha finito
                    if producer_finished.load(Ordering::SeqCst) {
                        break;
                    }
                }
            }
        }
    });

    consumer.join().unwrap();
}
