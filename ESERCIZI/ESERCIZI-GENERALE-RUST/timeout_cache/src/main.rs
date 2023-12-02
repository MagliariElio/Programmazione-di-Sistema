/*
Cache Thread-Safe con Timeout

Devi implementare una struttura dati chiamata TimeoutCache che consenta di archiviare e recuperare valori associati a chiavi con un
timeout. La struttura dovrebbe essere thread-safe e permettere la lettura e la scrittura concorrente.

La TimeoutCache deve offrire i seguenti metodi:

    new() -> Self: Crea una nuova istanza della cache.

    insert(key: K, value: V, timeout: Duration) -> Result<(), CacheError>: Inserisce un valore associato alla chiave
        specificata nella cache con un timeout. Se il valore è già presente, il suo timeout viene aggiornato. Se la cache è piena,
        attende finché non si libera spazio o finché non scade il timeout di attesa.

    get(&self, key: &K) -> Option<V>: Recupera il valore associato alla chiave specificata. Se il
        valore è scaduto, viene rimosso dalla cache e restituito None.

    remove(&self, key: &K) -> Option<V>: Rimuove il valore associato alla chiave specificata dalla cache e lo restituisce, se presente.

    clear(&self): Svuota completamente la cache.

    len(&self) -> usize: Restituisce il numero di elementi attualmente presenti nella cache.

    is_empty(&self) -> bool: Restituisce true se la cache è vuota, false altrimenti.

La TimeoutCache dovrebbe gestire in modo appropriato le operazioni concorrenti e il rilascio dei valori scaduti.
Assicurati di implementare anche eventuali meccanismi di attesa limitata nel tempo in caso di cache piena o
quando si attende che scada un timeout.
*/

use std::collections::HashMap;
use std::fmt::{Error};
use std::hash::Hash;
use std::ops::DerefMut;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Cache<V:Clone> {
    value: V,
    timeout: Instant,
}

#[derive(Debug)]
struct TimeoutCache<K: Eq + PartialEq + Hash, V:Clone> {
    cache_map: Mutex<HashMap<K, Cache<V>>>,
    cvar: Condvar,
}

impl<K: Eq + PartialEq + Hash, V: Clone> TimeoutCache<K, V> {
    fn new() -> Self {
        Self {
            cache_map: Mutex::new(HashMap::new()),
            cvar: Condvar::new(),
        }
    }

    fn insert(&self, key: K, value: V, timeout: Duration) -> Result<(), Error> {
        let mut cache_map = self.cache_map.lock().unwrap();
        let length = cache_map.len();
        cache_map = self
            .cvar
            .wait_timeout_while(cache_map, timeout, |value| value.len() == length)
            .unwrap()
            .0;

        cache_map.insert(
            key,
            Cache {
                value,
                timeout: Instant::now() + timeout,
            },
        );

        self.cvar.notify_all();
        return Ok(());
    }

    fn get(&self, key: &K) -> Option<V> {
        let mut cache_map = self.cache_map.lock().unwrap();
        let mut cache = cache_map.get_mut(key);
        match cache {
            Some(cache) => {
                let value = cache.value.clone();
                if Instant::now() >= cache.timeout {
                    cache_map.remove(key);
                }
                Some(value)
            },
            None => None
        }
    }

    fn remove(&self, key: &K) -> Option<V>{
        let mut cache_map = self.cache_map.lock().unwrap();
        let value = cache_map.remove(key);
        match value {
            Some(result) => Some(result.value),
            None => None
        }
    }

    fn clear(&self) {
        let mut cache_map = self.cache_map.lock().unwrap();
        cache_map.clear();
    }

    fn len(&self) -> usize{
        let mut cache_map = self.cache_map.lock().unwrap();
        cache_map.len()
    }

    fn is_empty(&self) -> bool {
        let mut cache_map = self.cache_map.lock().unwrap();
        cache_map.is_empty()
    }
}

fn main() {
    let cache = Arc::new(TimeoutCache::<&str, &str>::new());

    let thread_cache = cache.clone();
    let insert_thread = thread::spawn(move || {
        thread_cache.insert("key1", "value1", Duration::from_secs(3)).unwrap();
        thread_cache.insert("key2", "value2", Duration::from_secs(1)).unwrap();
        thread_cache.insert("key1", "value3", Duration::from_secs(7)).unwrap();
        thread_cache.insert("key2", "value4", Duration::from_secs(8)).unwrap();
    });

    let thread_cache = cache.clone();
    let get_thread = thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        let value = thread_cache.get(&"key1");
        println!("Value from thread: {:?}", value);
    });

    insert_thread.join().unwrap();
    get_thread.join().unwrap();

    println!("Cache length: {}", cache.len());
    println!("Cache length: {:?}", cache);
}
