use std::sync::{Arc, Condvar, Mutex};

/**
All'interno di un programma è necessario garantire che non vengano eseguite
CONTEMPORANEAMENTE più di N invocazioni di operazioni potenzialmente lente.
A questo scopo, è stata definita la struttura dati ExecutionLimiter che viene inizializzata con il
valore N del limite. Tale struttura è thread-safe e offre solo il metodo pubblico generico execute(f),
che accetta come unico parametro una funzione f, priva di parametri che ritorna il tipo generico R.
Il metodo execute(...) ha, come tipo di ritorno, lo stesso tipo R restituito da f ed ha il compito di
mantere il conteggio di quante invocazioni sono in corso. Se tale numero è già pari al valore N
definito all'atto della costruzione della struttura dati, attende, senza provocare consumo di CPU,
che scenda sotto soglia, dopodiché invoca la funzione f ricevuta come parametro e ne restituisce il
valore. Poiché l'esecuzione della funzione f potrebbe fallire, in tale caso, si preveda di
decrementare il conteggio correttamente.
Si implementi, usando i linguaggi Rust o C++, tale struttura dati, garantendo tutte le funzionalità
richieste.
*/

struct ExecutionLimiter <R> {
    count: Arc<((Mutex<usize>, Condvar))>,
    N: usize
}

impl <R> ExecutionLimiter<R> {
    fn new(limit: usize) -> Self {
        Self{
            count: Arc::new((Mutex::new(0), Condvar::new())),
            N: limit
        }
    }

    fn execute(&self, f: fn()->R) -> R {
        let (lock, cv) = &*self.count;

        let mut count = lock.lock().unwrap();

        while *count >= self.N {
            count = self.cv.wait(count).unwrap();
        }

        *count += 1;

        drop(lock);

        let result = f();

        count = lock.lock().unwrap();
        *count -= 1;

        self.cv.notify_one();

        return result;
    }
}

fn main() {

}
