use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

const MAX_LENGTH_TAPE: usize = 10;

struct CircularBuffer {
    tape: Vec<i32>,
    ind_r: usize,
    ind_w: usize,
    len: usize,
    max_len: usize
}

impl CircularBuffer {
    fn new() -> Self {
        let mut buffer = CircularBuffer {
            tape: Vec::<i32>::new(),
            ind_r: 0,
            ind_w: 0,
            len: 0,
            max_len: MAX_LENGTH_TAPE
        };

        for _ in 0..buffer.max_len {
            buffer.tape.push(-1);
        }

        return buffer;
    }

    fn insert(&mut self, value: i32) {
        if self.len < self.max_len {
            self.tape[self.ind_w] = value;
            self.ind_w = (self.ind_w + 1) % self.max_len;
            self.len += 1;
        }
    }

    fn read(&mut self) -> String {
        let mut result = String::new();

        while self.tape[self.ind_r] != -1 {
            result.push_str(format!("{} ", self.tape[self.ind_r]).as_str());
            self.tape[self.ind_r] = -1;
            self.ind_r = (self.ind_r + 1) % self.max_len;
            self.len -= 1;
        }
        return result;
    }
}

fn main() {
    let buf = Arc::new(RwLock::new(CircularBuffer::new()));

    let buf_producer = buf.clone();
    let buf_consumer = buf.clone();

    let producer = thread::spawn(move || {
        let mut i = 0;
        loop {
            let mut prod_buf = buf_producer.write().unwrap();
            prod_buf.insert(i);
            i += 1;
            drop(prod_buf);
            thread::sleep(Duration::from_secs(3));
        }
    });

    let consumer = thread::spawn(move || {
        loop {
            let mut buffer = buf_consumer.write().unwrap();

            println!("{:?}", buffer.tape);
            let value = buffer.read();
            println!("The producer has written: {}", value);
            println!("{:?}\n", buffer.tape);

            drop(buffer);
            thread::sleep(Duration::from_secs(10));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
