extern crate core;

use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

struct Element<T: Send + 'static + Clone> {
    value: T,
    time: Instant,
}

struct DelayedQueue<T: Send + 'static + Clone> {
    coda: Mutex<Vec<Element<T>>>,
    cvar: Condvar,
}

impl<T: Send + 'static + Clone> DelayedQueue<T> {
    fn new() -> Self {
        Self {
            coda: Mutex::new(Vec::new()),
            cvar: Condvar::new(),
        }
    }

    fn offer(&self, t: T, i: Instant) {
        let mut coda = self.coda.lock().unwrap();
        coda.push(Element { value: t, time: i });
        self.cvar.notify_all();
    }

    fn take(&self) -> Option<T> {
        let mut coda = self.coda.lock().unwrap();
        if coda.len() == 0 {
            return None;
        }

        let length = coda.len();
        let mut min = coda.get(0).clone().unwrap();
        let mut min_pos = 0;
        let current = Instant::now();

        for i in 0..coda.len() {
            if (coda[i].time.duration_since(current).as_nanos() as u64)
                < (min.time.duration_since(current).as_nanos() as u64)
            {
                min = &coda[i];
                min_pos = i;
            }
        }

        if min.time - current > Default::default() {
            coda = self.cvar.wait_timeout_while(coda, min.time.elapsed(), |value| {(min.time - current) > Default::default() && value.len() == length}).unwrap().0;
        }

        coda.remove(min_pos);

        return Some(min.value.clone());
    }

    fn size(&self) -> usize {
        let coda = self.coda.lock().unwrap();
        return coda.len();
    }
}

fn main() {
    println!("Hello, world!");
}
