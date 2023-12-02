use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

#[derive(Debug)]
struct BufferCircolare<T> {
    buffer: Vec<T>,
    ind_r: usize,
    ind_w: usize,
    len: usize,
    capacity: usize,
}

impl<T: Clone> BufferCircolare<T> {
    fn new(capacity: usize, default_value: T) -> Self {
        Self {
            buffer: vec![default_value; capacity],
            ind_r: 0,
            ind_w: 0,
            len: 0,
            capacity,
        }
    }

    fn insert(&mut self, value: T) {
        if self.len < self.capacity {
            self.buffer[self.ind_w] = value;
            self.ind_w = (self.ind_w + 1) % self.capacity;
            self.len += 1;
        }
    }

    fn read(&mut self) -> Option<T> {
        if self.len > 0 {
            let value = self.buffer[self.ind_r].clone();
            self.ind_r = (self.ind_r + 1) % self.capacity;
            self.len -= 1;
            return Some(value);
        }
        return None;
    }
}

fn main() {
    let buffer = Arc::new(Mutex::new(BufferCircolare::<i32>::new(10, 0)));

    let buffer_clone = buffer.clone();
    let producer = thread::spawn(move || {
        loop {
            let mut buf = buffer_clone.lock().unwrap();
            for _ in 0..buf.capacity {
                buf.insert(rand::thread_rng().gen_range(10..30));
            }
        }
    });

    let buffer_clone = buffer.clone();
    let consumer = thread::spawn(move || {
        loop {
            let mut buf = buffer_clone.lock().unwrap();
            for _ in 0..buf.capacity {
                buf.read();
                println!("{:?}", buf);
            }
            drop(buf);
            thread::sleep(Duration::from_secs(10));
            println!();
        }
    });


    producer.join().unwrap();
    consumer.join().unwrap();
}
