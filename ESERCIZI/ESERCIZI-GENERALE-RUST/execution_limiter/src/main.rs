use std::{panic, thread};
use std::panic::UnwindSafe;
use std::sync::{Arc, Condvar, Mutex};

struct ExecutionLimiter {
    count: Mutex<u32>,
    limit: u32,
    cvar: Condvar,
}

impl ExecutionLimiter {
    fn new(n: u32) -> Self {
        Self {
            count: Mutex::new(0),
            limit: n,
            cvar: Condvar::new(),
        }
    }

    fn execute<R>(&self, f: impl Fn() -> R + UnwindSafe) -> Option<R> {
        let mut count = self.count.lock().unwrap();
        count = self.cvar.wait_while(count, |value| *value == self.limit).unwrap();
        *count += 1;

        drop(count);
        let res = panic::catch_unwind(f);

        let mut count = self.count.lock().unwrap();
        *count -= 1;
        self.cvar.notify_all();
        return match res {
            Ok(result) => Some(result),
            Err(_) => None
        }
    }
}

fn main() {
    let execution_limiter = Arc::new(ExecutionLimiter::new(2));
    let mut thread_vec = Vec::new();

    for i in 0..10 {
        let execution = execution_limiter.clone();
        thread_vec.push(thread::spawn(move || {
            let res = execution.execute(|| {println!("Stampa thread {}", i); return i;});
            println!("{}", res.unwrap());
        }))
    }

    for thread in thread_vec {
        thread.join().unwrap();
    }
}
