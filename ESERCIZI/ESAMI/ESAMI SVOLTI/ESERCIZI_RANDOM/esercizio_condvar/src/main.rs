use std::sync::mpsc::channel;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/**
Scrivi un programma in Rust che utilizzi una condizione di variabile (condvar) per sincronizzare il lavoro di due thread.
Il primo thread deve stampare sulla console i numeri da 1 a 10, mentre il secondo thread
deve stampare i numeri da 11 a 20. Assicurati che i due thread si alternino l'esecuzione in modo da stampare i numeri in ordine crescente.

 */

fn main() {
    let mutex_version = false;

    if mutex_version {
        function_mutex_version();
    } else {
        function_channel_version();
    }
}

fn function_mutex_version() {
    let pair = Arc::new((Mutex::new(0), Condvar::new()));
    let (tx, rx) = channel();

    let pair_t1 = pair.clone();
    let t1 = thread::spawn(move || {
        let (lock, cvar) = &*pair_t1;

        let mut num = lock.lock().unwrap();
        tx.send("go").unwrap();
        for _ in 1..11 {
            *num += 1;
            println!("Thread 1: {}", *num);

            cvar.notify_one();
            num = cvar.wait(num).unwrap();
        }
        cvar.notify_one();
    });

    let pair_t2 = pair.clone();
    let t2 = thread::spawn(move || {
        let (lock, cvar) = &*pair_t2;

        match rx.recv() {
            Ok(_) => {}
            Err(_) => {}
        }
        let mut num = lock.lock().unwrap();
        for _ in 11..21 {
            *num += 1;
            println!("Thread 2: {}", *num);

            cvar.notify_one();
            num = cvar.wait(num).unwrap();
        }
        cvar.notify_one();
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

fn function_channel_version() {
    let (tx_1, rx_1) = channel();
    let (tx_2, rx_2) = channel();

    let t1 = thread::spawn(move || {
        let mut num = 0;
        for _ in 1..11 {
            num += 1;
            println!("Thread 1: {}", num);

            tx_2.send(num).unwrap();
            match rx_1.recv() {
                Ok(result) => {
                    num = result;
                }
                Err(_) => {}
            }
        }
    });

    let t2 = thread::spawn(move || {
        let mut num = 0;
        for _ in 11..21 {

            println!("len: {}", rx_2.recv().)

            match rx_2.recv() {
                Ok(result) => {
                    num = result;
                }
                Err(_) => {}
            }
            num += 1;
            println!("Thread 2: {}", num);
            tx_1.send(num).unwrap();
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
