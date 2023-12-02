use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::{Arc, Condvar, Mutex};

#[derive(PartialEq)]
enum Status {
    Calling,
    Free,
}

struct Cache<K, V>
where
    K: Display + Clone + Eq + PartialEq + Hash,
    V: Clone + Display
{
    map: Mutex<(HashMap<K, Arc<V>>, Status)>,
    cvar: Condvar,
}

impl<K, V> Cache<K, V>
where
    K: Display + Clone + Eq + PartialEq + Hash,
    V: Clone + Display
{
    fn new() -> Self {
        Self {
            map: Mutex::new((HashMap::new(), Status::Free)),
            cvar: Condvar::new(),
        }
    }

    fn get<F>(&self, key: K, f: F) -> Arc<V>
    where
        F: Fn(K) -> V,
    {
        let mut map = self.map.lock().unwrap();

        return match map.0.get(&key) {
            Some(v) => v.clone(),
            None => {
                map = self.cvar.wait_while(map, |value| value.1 == Status::Calling).unwrap();
                map.1 = Status::Calling;
                drop(map);
                let result = Arc::new(f(key.clone()));

                let mut map = self.map.lock().unwrap();
                map.0.insert(key, result.clone());

                map.1 = Status::Free;
                self.cvar.notify_all();

                result.clone()
            }
        };
    }
}

fn main() {
    println!("Hello, world!");
}
