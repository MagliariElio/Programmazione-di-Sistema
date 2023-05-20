#[warn(unused_imports)]
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;

struct BarrierState {
    count: usize,
    generation_id: usize,       // serve ad evitare lo spurious wakeup, ovvero risvegli improvvisi non previsti
}

struct CyclicBarrier {
    lock: Mutex<BarrierState>,
    cvar: Condvar,
    num_threads: usize,
}

impl CyclicBarrier {
    fn new(number: usize) -> Self {
        Self {
            num_threads: number,
            lock: Mutex::new(BarrierState { count: 0, generation_id: 0}),
            cvar: Condvar::new(),
        }
    }

    fn wait(&self) {
        let mut lock = self.lock.lock().unwrap();
        lock.count += 1;
        let local_generation = lock.generation_id;

        if lock.count < self.num_threads {
            while local_generation == lock.generation_id {
                lock = self.cvar.wait(lock).unwrap(); // quando il thread entra qui rilascia il mutex e si sospende sulla condition variable
            }
        } else {
            lock.count = 0;
            lock.generation_id = lock.generation_id.wrapping_add(1);
            self.cvar.notify_all();
        }
    }
}

fn main() {
    const NUMBER_THREAD: usize = 3;
    let barrier = Arc::new(CyclicBarrier::new(NUMBER_THREAD));
    let mut vt = Vec::new();

    for i in 0..NUMBER_THREAD {
        let cbarrier = barrier.clone();
        vt.push(thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("Value {} from thread {}", j, i);
            }
        }))
    }

    for j in vt {
        j.join().unwrap();
    }
}
