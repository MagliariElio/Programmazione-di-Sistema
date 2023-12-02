use std::rc::Rc;
use rand::Rng;
use std::sync::mpsc::channel;
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Sensor {
    value: u32,
}

impl Sensor {
    fn read_value(&mut self) -> u32 {
        let value = rand::thread_rng().gen_range(0..10);
        self.value = value;
        value
    }
}

struct Machine {
    sensors: Vec<Sensor>,
    count: u32,
    speed: u64,
}

impl Machine {
    fn new() -> Self {
        Self {
            sensors: vec![Sensor { value: 0 }; 10],
            count: 10,
            speed: 10,
        }
    }

    fn set_speed(&mut self, speed: u64) {
        self.speed = speed;
    }
}


fn main() {
    let machine = Arc::new(Mutex::new(Machine::new()));
    let cvar = Arc::new(Condvar::new());
    let barrier = Arc::new(Barrier::new(11));
    let mut threads_vec = Vec::new();
    let (sender, receiver) = channel();

    for i in 0..10 {
        let machine_clone = machine.clone();
        let sender_clone = sender.clone();
        let cvar_cloned = cvar.clone();
        let barrier_cloned = barrier.clone();
        threads_vec.push(thread::spawn(move || loop {
            let mut machine = machine_clone.lock().unwrap();
            let value = machine.sensors[i].read_value();

            if let Err(err) = sender_clone.send(value) {
                println!("sender {} got an error: {}", i, err);
            } else {
                println!("sender {} has written: {}", i, value);
            }

            machine.count -= 1;
            cvar_cloned.notify_all();
            let speed = machine.speed;
            drop(machine);
            barrier_cloned.wait();
            thread::sleep(Duration::from_secs(speed));
        }));
    }

    let machine_clone = machine.clone();
    let cvar_cloned = cvar.clone();
    let barrier_cloned = barrier.clone();
    let thread_reader = thread::spawn(move || loop {
        let mut machine = machine_clone.lock().unwrap();
        machine = cvar_cloned
            .wait_while(machine, |value| value.count != 0)
            .unwrap();
        let mut sum = 0;
        match receiver.try_recv() {
            Ok(value) => {
                println!("reader: {}", value);
                sum += value;
            },
            Err(_) => {
                machine.count = 10;
                let speed = machine.speed;
                if sum >= 50 {
                    machine.set_speed(speed + 1);
                } else {
                    machine.set_speed(speed - 1);
                }
                barrier_cloned.wait();
            }
        }
    });

    for thread in threads_vec {
        thread.join().unwrap();
    }
    thread_reader.join().unwrap();
}
