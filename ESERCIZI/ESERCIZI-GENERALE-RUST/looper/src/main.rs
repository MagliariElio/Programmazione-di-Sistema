
/*
Devi implementare un'astrazione chiamata Looper in Rust, che consenta l'elaborazione di messaggi in un thread dedicato.
La struttura Looper deve soddisfare i seguenti requisiti:

    new(process: fn(Message), cleanup: fn()) -> Looper: Crea una nuova istanza di Looper con due funzioni passate come argomenti.
        La funzione process sarà responsabile dell'elaborazione dei messaggi, accettando un parametro di tipo Message e non restituendo
        alcun valore. La funzione cleanup sarà chiamata quando il thread di Looper sta per terminare e non accetta argomenti.

    send(&self, message: Message) -> Result<(), String>: Invia un messaggio alla coda del Looper. Se la coda è piena, questa
        funzione dovrebbe restituire un errore con un messaggio appropriato. In caso contrario, il messaggio dovrebbe essere
        inserito nella coda del Looper.

    start(&self) e stop(&self) -> Result<(), String>: Avvia e ferma il thread del Looper. Il thread dovrebbe iniziare l'elaborazione
        dei messaggi solo quando start viene chiamato e dovrebbe essere interrotto quando stop viene chiamato. La funzione stop dovrebbe
        attendere che il thread termini prima di restituire un risultato. Se il thread è già in esecuzione quando si chiama start, questa
        funzione dovrebbe restituire un errore con un messaggio appropriato. In caso contrario, dovrebbe iniziare il thread.

Assicurati che le operazioni sulla coda dei messaggi siano thread-safe e che il thread di Looper termini correttamente quando richiesto.
*/

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(PartialEq)]
enum Status {
    Started,
    Execution,
    Stopped
}

struct Queue<Message: Send + 'static> {
    queue: VecDeque<Message>,
    status: Status
}

struct Looper<Message: Send + 'static> {
    queue_looper: Mutex<Queue<Message>>,
    cleanup: fn(),
    cvar: Condvar,
    n: usize
}

impl<Message: Send + 'static> Looper<Message> {
    fn new(process: fn(Message), cleanup: fn(), n: usize) -> Arc<Self> {
        let looper = Arc::new(Self {
            queue_looper: Mutex::new(Queue {queue: VecDeque::with_capacity(n), status: Status::Stopped}),
            cleanup,
            cvar: Condvar::new(),
            n
        });
        let looper_cloned = looper.clone();
        thread::spawn(move ||
            {
                loop {
                    let mut queue = looper_cloned.queue_looper.lock().unwrap();

                    queue = looper_cloned.cvar.wait_while(queue, |value| value.status == Status::Stopped).unwrap();
                    queue.status = Status::Execution;

                    for _ in 0..queue.queue.len() {
                        match queue.queue.pop_front() {
                            Some(element) => process(element),
                            None => {}
                        }
                    }

                    queue.status = Status::Started;

                    queue = looper_cloned.cvar.wait_while(queue, |value| value.status == Status::Started && queue.queue.len() == 0).unwrap();
                }
            });

        return looper;
    }

    fn send(&self, message: Message) -> Result<(), String> {
        let mut queue = self.queue_looper.lock().unwrap();
        if self.n == queue.queue.len() {
            return Err("The queue is full".to_string());
        }
        queue.queue.push_back(message);
        return Ok(());
    }


    fn start(&self) -> Result<(), String> {
        let mut queue = self.queue_looper.lock().unwrap();
        if queue.status == Status::Execution {
            return Err("Thread is already started".to_string());
        }

        queue.status = Status::Started;
        self.cvar.notify_all();
        return Ok(());
    }

    fn stop(&self) -> Result<(), String> {
        let mut queue = self.queue_looper.lock().unwrap();

        queue = self.cvar.wait_while(queue, |value| value.status == Status::Execution).unwrap();

        queue.status = Status::Stopped;
        self.cvar.notify_all();
        return Ok(());
    }

}

impl<Message: Send + 'static> Drop for Looper<Message> {
    fn drop(&mut self) {
        (self.cleanup)();
        drop(self);
    }
}






fn main() {
    println!("Hello, world!");
}
