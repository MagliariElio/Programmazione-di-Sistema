#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::sync::mpsc::sync_channel;
use std::sync::{mpsc, Arc, Barrier, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[derive(Copy, Clone)]
struct Sensor {
    value: u32,
}

impl Sensor {
    fn new(v: u32) -> Self {
        Self { value: v }
    }

    fn read_value(&mut self, speed: u32) -> u32 {
        let sleep_value = thread_rng().gen_range(0..=10) as u32 + speed;
        sleep(Duration::from_secs(sleep_value as u64));
        self.value = sleep_value;
        return sleep_value;
    }
}

struct Machine {
    v_sensor: [Sensor; 10],
    speed: u32,
}

const MIN_SPEED: u32 = 0;
const MAX_SPEED: u32 = 10;

impl Machine {
    fn new() -> Machine {
        Self {
            v_sensor: [Sensor::new(0); 10],
            speed: MAX_SPEED / 2,
        }
    }

    fn set_speed(&mut self, mut speed: u32) {
        while speed < MIN_SPEED {
            speed += 1;
        }

        while speed > MAX_SPEED {
            speed -= 1;
        }

        if speed >= MIN_SPEED && speed <= MAX_SPEED {
            self.speed = speed;
        }
    }

    fn sum(&self) -> u32 {
        let mut result = 0;
        for sensor in self.v_sensor {
            result += sensor.value;
        }

        return result;
    }
}

#[cfg(feature = "barrier")]
// per avviarlo bisogna mettere nel command --features barrier
fn main() {
    const THREAD_NUMBER: usize = 11;
    let arc_machine = Arc::new(Mutex::new(Machine::new()));
    let arc_barrier = Arc::new(Barrier::new(THREAD_NUMBER)); // 10 thread in lettura + 1 thread in scrittura
    let arc_barrier_sum = Arc::new(Barrier::new(THREAD_NUMBER)); // 10 thread in lettura + 1 thread in scrittura

    let mut v_thread = Vec::new();

    for i in 0..THREAD_NUMBER - 1 {
        let machine_cloned = arc_machine.clone();
        let barrier = arc_barrier.clone();
        let barrier_sum = arc_barrier_sum.clone();
        v_thread.push(thread::spawn(move || {
            let mut iteration = 0;
            loop {
                barrier.wait(); // creo una barriera così tutti i thread leggono insieme

                let mut machine = machine_cloned.lock().unwrap();
                let speed = machine.speed;
                let sensor = machine.v_sensor.get_mut(i).unwrap();
                let value = sensor.read_value(speed);

                iteration += 1;
                println!("iteration {}: sensor {} wrote {}", iteration, i, value);

                drop(machine); // rilascio del lock
                barrier_sum.wait(); // mi metto in attesa che anche gli altri thread abbiano letto così da poter fare la somma
            }
        }))
    }

    let thread_s = thread::spawn(move || {
        let machine_cloned = arc_machine.clone();
        let barrier = arc_barrier.clone();
        let barrier_sum = arc_barrier_sum.clone();
        loop {
            barrier.wait(); // dà il via alla lettura

            barrier_sum.wait(); // aspetto che tutti i thread abbiano letto e poi faccio la somma

            let mut machine = machine_cloned.lock().unwrap();
            let value = machine.sum();
            let mut speed = machine.speed;

            if value > 50 {
                speed += 1;
                machine.set_speed(speed); // rallenta
            } else {
                if speed > 0 {
                    speed -= 1;
                }
                machine.set_speed(speed); // accelera
            }

            println!("The sum is {} so the new speed is {}\n", value, speed);
        }
    });

    for v in v_thread {
        v.join().unwrap();
    }

    thread_s.join().unwrap();
}

#[cfg(not(feature = "barrier"))]
fn main() {
    const THREAD_NUMBER: usize = 11;
    let arc_machine = Arc::new(Mutex::new(Machine::new()));
    let (sender, receiver) = sync_channel(THREAD_NUMBER);
    let condvar = Arc::new(Condvar::new());

    let mut v_thread = Vec::new();

    for i in 0..THREAD_NUMBER - 1 {
        let machine_cloned = arc_machine.clone();
        let sender_cloned = sender.clone();
        let condvar_cloned = condvar.clone();
        v_thread.push(thread::spawn(move || {
            let mut iteration = 0;
            loop {
                let mut machine = machine_cloned.lock().unwrap();
                let speed = machine.speed;
                let sensor = machine.v_sensor.get_mut(i).unwrap();
                let value = sensor.read_value(speed);

                iteration += 1;
                sender_cloned
                    .send(format!(
                        "iteration {}: sensor {} wrote {}",
                        iteration, i, value
                    ))
                    .unwrap();

                let _machine = condvar_cloned.wait(machine).unwrap();
            }
        }))
    }

    let thread_s = thread::spawn(move || {
        let machine_cloned = arc_machine.clone();
        loop {
            for _ in 0..THREAD_NUMBER - 1 {
                match receiver.recv() {
                    Ok(message) => {
                        println!("received => {}", message);
                    }
                    Err(mpsc::RecvError) => {
                        println!("Error");
                    }
                }
            }

            let mut machine = machine_cloned.lock().unwrap();
            let value = machine.sum();
            let mut speed = machine.speed;

            if value > 50 {
                speed += 1;
                machine.set_speed(speed); // rallenta
            } else {
                if speed > 0 {
                    speed -= 1;
                }
                machine.set_speed(speed); // accelera
            }

            println!("The sum is {} so the new speed is {}\n", value, speed);

            condvar.notify_all(); // avvisa tutti i thread consumer di poter procedere
        }
    });

    for v in v_thread {
        v.join().unwrap();
    }

    thread_s.join().unwrap();
}
