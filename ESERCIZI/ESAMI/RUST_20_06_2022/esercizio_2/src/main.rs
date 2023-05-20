/*
    Un paradigma frequentemente usato nei sistemi reattivi è costituito dall'astrazione detta Looper.
    Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread.
    Il thread attende, senza consumare cicli di CPU, che siano presenti messaggi nella coda, li estrae a uno a uno nell'ordine di arrivo, e li elabora.

    Il costruttore di Looper riceve due parametri, entrambi di tipo puntatore a funzione: process(...) e cleanup().

    La prima è una funzione responsabile di elaborare i singoli messaggi ricevuti attraverso la coda; tale funzione accetta un
    unico parametro in ingresso di tipo Message e non ritorna nulla;

    La seconda è una funzione priva di argomenti e valore di ritorno, e verrà invocata dal thread incapsulato nel Looper quando esso starà per terminare.

    Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per
    gestirne il ciclo di vita: send(msg), che accetta come parametro un oggetto generico di tipo
    Message che verrà inserito nella coda e successivamente estratto dal thread ed inoltrato alla
    funzione di elaborazione.

    Quando un oggetto Looper viene distrutto, occorre fare in modo che ii thread contenuto al suo interno invochi la seconda funzione passata nel costruttore e poi termini.

    Si implementi, utilizzando ii linguaggio Rust o C++, tale astrazione tenendo canto che i suoi metodi dovranno essere thread-safe.
*/

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

type Message = String;

struct Looper {
    sender: Sender<Message>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Looper {
    pub fn new<F: Fn(Message) + Send + 'static, G: Fn() + Send + 'static>(
        process_fn: F,
        cleanup_fn: G,
        sender: Sender<Message>,
        receiver: Receiver<Message>,
    ) -> Self {
        let thread = thread::spawn(move || {
            loop {
                match receiver.try_recv() {
                    Ok(message) => { process_fn(message); }
                    Err(_) => { cleanup_fn(); return; }
                }
            }
            /*while let Ok(message) = receiver.recv() {
                process_fn(message);
            }*/
        });

        Looper {
            sender,
            thread: Some(thread),
        }
    }

    pub fn send(&self, message: Message) {
        self.sender.send(message).unwrap();
    }
}

impl Drop for Looper {
    fn drop(&mut self) {
        println!("Looper Drop");
        // Chiudiamo il canale di comunicazione e attendiamo che il thread termini
        drop(&self.sender);
        if let Some(handle) = self.thread.take() {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let (sender, receiver) = channel::<Message>();
    let process = |message: Message| println!("Message Received: {}", message);
    let cleanup = || println!("Cleanup");
    let looper = Looper::new(process, cleanup, sender, receiver);

    looper.send("Hello".to_string());
    looper.send("Hi".to_string());
    looper.send("Ciao".to_string());
    looper.send("Bye".to_string());
    looper.send("Salut".to_string());
    looper.send("Hola".to_string());

}
