use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::process::exit;

#[derive(Debug)]
struct Calendar {
    schedule: Vec<(f64, f64)>,
    bounds: (f64, f64),
}

impl Calendar {
    fn new(min: f64, max: f64) -> Self {
        Self {
            schedule: Vec::new(),
            bounds: (min, max)
        }
    }

    fn insert(&mut self, min: f64, max: f64) {
        let bound = (min, max);
        self.schedule.push(bound);
    }

}

fn main() {
    let mut name_1= "";
    let mut name_2= "";
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        name_1 = &args[1].as_str();
        name_2 = &args[2].as_str();
    } else {
        eprintln!("Nessun argomento passato");
        exit(-1);
    }

    let mut schedule: Vec<f64> = read_from_file(name_1);
    let mut calendar_1 = Calendar::new(schedule[0], schedule[1]);
    schedule = schedule.split_off(2);

    for pair in schedule.chunks_exact(2) {
        calendar_1.insert(pair[0], pair[1]);
    }

    println!("{:?}", calendar_1);

    schedule = read_from_file(name_2);
    let mut calendar_2 = Calendar::new(schedule[0], schedule[1]);
    schedule = schedule.split_off(2);

    for pair in schedule.chunks_exact(2) {
        calendar_2.insert(pair[0], pair[1]);
    }

    println!("{:?}", calendar_2);

    check_free_range_calendar(calendar_1, calendar_2);
}

fn read_from_file(name: &str) -> Vec<f64> {
    let input = File::open(name).unwrap();
    let buffered = BufReader::new(input);
    let mut schedule: Vec<f64> = Vec::new();

    for line in buffered.lines() {
        let hour = match line {
            Ok(x) => x,
            Err(_) => exit(-1),
        };

        let hours: Vec<&str> = hour.split(":").collect();

        let hours_vec: Vec<f64> = hours
            .iter()
            .map(|&x| {
                let y = x.parse::<f64>();
                if y.is_ok() {
                    y.unwrap()
                } else {
                    panic!("Error in the parsing");
                }
            })
            .collect();

        let sum = hours_vec.get(0).unwrap() + hours_vec.get(1).unwrap() * 0.01;
        schedule.push(sum);
    }

    return schedule;
}

fn check_free_range_calendar(calendar_1: Calendar, calendar_2: Calendar) {
    let schedule_1 = calendar_1.schedule;
    let schedule_2 = calendar_2.schedule;
    let mut min = 0.0;
    let mut max = 0.0;

    if calendar_1.bounds.0 >= calendar_2.bounds.0 {
        if calendar_1.bounds.1 <= calendar_2.bounds.1 {
            min = calendar_1.bounds.0;
            max = calendar_1.bounds.1;
        } else {
            min = calendar_1.bounds.0;
            max = calendar_2.bounds.1;
        }
    } else {
        if calendar_1.bounds.1 > calendar_2.bounds.1 {
            min = calendar_2.bounds.0;
            max = calendar_2.bounds.1;
        } else {
            min = calendar_2.bounds.0;
            max = calendar_1.bounds.1;
        }
    }

    let mut free_schedule = create_free_schedule(min, max);
    free_schedule = check_free_range(schedule_1.clone(), free_schedule, min, max);
    free_schedule = check_free_range(schedule_2.clone(), free_schedule, min, max);

    println!("risultato = {:?}", free_schedule);

}

fn check_free_range(schedule: Vec<(f64, f64)>, mut free_schedule: Vec<f64>, mut min: f64, max:f64) -> Vec<f64>{

    for pair_1 in &schedule {
        let mut min_pair = pair_1.0 + 0.3;
        let mut max_pair = pair_1.1;

        if min_pair.fract() >= 0.6 {
            min_pair += 0.4;
            min_pair = (min_pair*100.0).trunc() / 100.0;
        }

        if min_pair < min {
            min_pair = min;
        }

        if max_pair > max {
            max_pair = max;
        }

        while min_pair < pair_1.1 {
            free_schedule = free_schedule.iter().filter(|&x| *x != min_pair).cloned().collect();

            if min_pair.fract() < 0.3 {
                min_pair += 0.3;
            } else if min_pair.fract() >= 0.3 {
                min_pair += 0.7;
            }

            min_pair = (min_pair*100.0).trunc() / 100.0;
        }
    }

    println!("{:?}", free_schedule);

    return free_schedule;
}

fn create_free_schedule(mut min: f64, max:f64) -> Vec<f64> {
    let mut free_schedule = Vec::<f64>::new();

    println!("min: {}, max: {}", min, max);

    let mut on = 1;
    while min <= max {
        free_schedule.push(min);
        if on == 1 {
            min += 0.3;
            on = 0;
        } else {
            min += 0.7;
            on = 1;
        }
    }

    println!("schedule = {:?}", free_schedule);
    return free_schedule;
}


/*

9:30    min
10:00   -
10:30   -
11:00   -
11:30   -
12:00
12:30
13:00
13:30
14:00
14:30
15:00
15:30
16:00
16:30
17:00
17:30
18:00   max

 */