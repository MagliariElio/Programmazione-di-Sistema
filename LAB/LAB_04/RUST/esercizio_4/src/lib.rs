use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;


fn count_letters(value: &str, map: &mut HashMap<char, usize>) {
    for ch in value.chars() {
        if !ch.is_alphabetic(){
            continue;
        }
        if map.contains_key(&ch) {
            *map.get_mut(&ch).unwrap() += 1;
        } else {
            map.insert(ch, 1);
        }
    }
}

fn union_maps(map_1: &HashMap<char, usize>, map_2: HashMap<char, usize>) -> HashMap<char, usize> {
    let mut result = map_1.clone();

    if map_1.len() > 0 {
        for key in map_1.keys() {
            if map_2.contains_key(key) {
                *result.get_mut(key).unwrap() += map_2.get(key).unwrap();
            }
        }
    } else if map_2.len() > 0 {
        result = map_2.clone();
    }

    return result;
}

pub fn frequency(inputs: &[&str], workers_count: usize) -> HashMap<char, usize>  {
    let input: &Vec<String> = &inputs.iter().map(|s| s.to_lowercase()).collect();
    let mutex = Arc::new(Mutex::new(HashMap::<char, usize>::new()));
    let worker_count = if workers_count > input.len() {input.len()} else {workers_count};

    if input.len() > 0 && worker_count > 0 {
        thread::scope(|s| {
            for i in 0..worker_count {
                let sem = mutex.clone();
                s.spawn(move || {
                    let mut rest = 0;
                    if i == worker_count - 1 {
                        rest = input.len() % worker_count;
                    }

                    let vec = &input[i*(input.len()/worker_count)..((i+1)*(input.len()/worker_count)+rest)];

                    let mut hash_map = HashMap::<char, usize>::new();
                    let _: Vec<_> = vec.iter().map(|value| count_letters(value.as_str(), &mut hash_map)).collect();

                    let mut result = sem.lock().unwrap();
                    *result = union_maps(&*result, hash_map);
                });
            }
        });
    }

    let res = mutex.lock();
    match res {
        Ok(map) => return map.clone(),
        Err(_) => HashMap::new()
    }
}

/*
    input length = 5
    worker count = 2
    2   2+1

    input length = 10
    worker count = 3
    3   3   3+1

    input length = 10
    worker count = 4
    2   2   2   2+2


*/
