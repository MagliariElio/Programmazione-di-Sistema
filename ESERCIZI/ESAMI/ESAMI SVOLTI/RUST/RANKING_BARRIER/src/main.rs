use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

/**
    Una barriera è un costrutto di sincronizzazione usato per regolare l'avanzamento relativo della computazione di più thread.
    All'atto della costruzione di questo oggetto, viene indicato il numero N di thread coinvolti. Non è lecito creare una barriera con meno di 2 thread.
    La barriera offre un solo metodo, wait(), il cui scopo è bloccare temporaneamente l'esecuzione del thread che lo ha invocato,
    non ritornando fino a che non sono giunte altre N-1 invocazioni dello stesso metodo da parte di altri thread: quando ciò succede,
    la barriera si sblocca e tutti tornano. Successive invocazioni del metodo wait() hanno lo stesso comportamento: la barriera è ciclica.
    Attenzione a non mescolare le fasi di ingresso e di uscita!
    Una RankingBarrier è una versione particolare della barriera in cui il metodo wait() restituisce un intero che rappresenta l'ordine di arrivo:
    il primo thread ad avere invocato wait() otterrà 1 come valore di ritorno, il secondo thread 2, e così via. All'inizio di un nuovo ciclo, il conteggio
    ripartirà da 1.

    Si implementi la struttura dati RankingBarrier a scelta in linguaggio Rust o C++ '11 o successivi.
*/

#[derive(PartialEq)]
enum Status {
    Filling,
    Emptying,
}

struct RankingBarrier {
    barrier: Mutex<(u32, Status)>,
    cvar: Condvar,
    max_len: u32,
}

impl RankingBarrier {
    fn new(max: u32) -> Self {
        Self {
            barrier: Mutex::new((0, Status::Filling)),
            cvar: Condvar::new(),
            max_len: max,
        }
    }

    fn wait(&self) {
        let mut barrier = self.barrier.lock().unwrap();
        thread::sleep(Duration::from_millis(500));
        barrier = self.cvar.wait_while(barrier, |value| value.1 == Status::Emptying).unwrap();
        barrier.0 += 1;

        if barrier.0 == self.max_len {
            barrier.0 -= 1;
            barrier.1 = Status::Emptying;
            self.cvar.notify_all();
        } else {
            barrier = self.cvar.wait_while(barrier, |value| value.1 == Status::Filling).unwrap();
            barrier.0 -= 1;

            if barrier.0 == 0 {
                barrier.1 = Status::Filling;
                self.cvar.notify_all();
                println!("--------------------");
            }
        }
    }
}

fn main() {
    let thread_max = 7;
    let barrier = Arc::new(RankingBarrier::new(thread_max));
    let mut thread_vec = Vec::new();

    for i in 0..thread_max+10 {
        let barrier_cloned = barrier.clone();
        thread_vec.push(thread::spawn(move || loop {
            barrier_cloned.wait();
            println!("Thread number {}", i);
        }))
    }

    for thread in thread_vec {
        thread.join().unwrap();
    }
}
