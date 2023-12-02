use std::sync::{Arc, Condvar, Mutex};

#[derive(PartialEq)]
enum Status {
    Emptying,
    Filling
}

struct CountState {
count: u32,
status: Status
}

struct RankingBarrier {
    barrier: Mutex<CountState>,
    n_threads: u32,
    cvar: Condvar
}

impl RankingBarrier {
    fn new(n_threads: u32) -> Self {
        Self {
            barrier: Mutex::new(CountState {count: 0, status: Status::Filling}),
            n_threads,
            cvar: Condvar::new()
        }
    }

    fn wait(&self) -> u32 {
        let mut barrier = self.barrier.lock().unwrap();
        barrier = self.cvar.wait_while(barrier, |value| value.status == Status::Emptying).unwrap();
        let count_wait = barrier.count;
        barrier.count += 1;

        if barrier.count == self.n_threads {
            barrier.status = Status::Emptying;
            barrier.count -= 1;
            self.cvar.notify_all();
        } else {
            barrier = self.cvar.wait_while(barrier, |value| value.status == Status::Filling).unwrap();
            barrier.count -= 1;

            if barrier.count == 0 {
                barrier.status = Status::Filling;
                self.cvar.notify_all();
            }
        }

        return count_wait;
    }
}

fn main() {
    let abarrrier = Arc::new(RankingBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let cbarrier = abarrrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                let count = cbarrier.wait();
                println!("after barrier\n thread: {} cycle: {} count: {}\n", i, j, count);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}
