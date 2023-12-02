/*
    La struct MpMcChannel<E: Send> è una implementazione di un canale su cui possono
    scrivere molti produttori e da cui possono attingere valori molti consumatori.
    Tale struttura offre i seguenti metodi:

    new(n: usize) -> Self //crea una istanza del canale basato su un buffer circolare di "n" elementi

    send(e: E) -> Option<()>
        //invia l'elemento "e" sul canale. Se il buffer circolare è pieno, attende
        //senza consumare CPU che si crei almeno un posto libero in cui depositare il valore
        //Ritorna:
        // - Some(()) se è stato possibile inserire il valore nel buffer circolare
        // - None se il canale è stato chiuso (Attenzione: la chiusura può avvenire anche
        // mentre si è in attesa che si liberi spazio) o se si è verificato un errore interno

    recv() -> Option<E>
        //legge il prossimo elemento presente sul canale. Se il buffer circolare è vuoto,
        //attende senza consumare CPU che venga depositato almeno un valore
        //Ritorna:
        // - Some(e) se è stato possibile prelevare un valore dal buffer
        // - None se il canale è stato chiuso (Attenzione: se, all'atto della chiusura sono
        // già presenti valori nel buffer, questi devono essere ritornati, prima di indicare
        // che il buffer è stato chiuso; se la chiusura avviene mentre si è in attesa di un valore,
        // l'attesa si sblocca e viene ritornato None) o se si è verificato un errore interno.

    shutdown() -> Option<()>
        //chiude il canale, impedendo ulteriori invii di valori.
        //Ritorna:
        // - Some(()) per indicare la corretta chiusura
        // - None in caso di errore interno all'implementazione del metodo.

    Si implementi tale struttura dati in linguaggio Rust, senza utilizzare i canali forniti dalla
    libreria standard né da altre librerie, avendo cura di garantirne la correttezza in
    presenza di più thread e di non generare la condizione di panico all'interno dei suoi
    metodi.
*/

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(PartialEq)]
enum Status {
    Opened,
    Closed,
}

struct Channel<E: Send> {
    channel: VecDeque<E>,
    status: Status,
}

struct MpMcChannel<E: Send> {
    mutex_channel: Mutex<Channel<E>>,
    cvar: Condvar,
    n: usize,
}

impl<E: Send> MpMcChannel<E> {
    fn new(n: usize) -> Arc<Self> {
        Arc::new(Self {
            mutex_channel: Mutex::new(Channel {
                channel: VecDeque::with_capacity(n),
                status: Status::Opened,
            }),
            cvar: Condvar::new(),
            n,
        })
    }

    fn send(&self, msg: E) -> Option<()> {
        let mut mutex_channel = match self.mutex_channel.lock() {
            Ok(res) => res,
            Err(_) => return None
        };

        mutex_channel = match self.cvar.wait_while(mutex_channel, |value| value.channel.len() == self.n && value.status == Status::Opened) {
            Ok(res) => res,
            Err(_) => return None
        };

        if mutex_channel.status == Status::Closed {
            return None;
        }

        mutex_channel.channel.push_back(msg);
        self.cvar.notify_all();
        Some(())
    }

    fn recv(&self) -> Option<E> {
        let mut mutex_channel = match self.mutex_channel.lock() {
            Ok(res) => res,
            Err(_) => return None
        };

        if mutex_channel.status == Status::Opened {
            mutex_channel = match self.cvar.wait_while(mutex_channel, |value| value.channel.len() == 0 && value.status == Status::Opened) {
                Ok(res) => res,
                Err(_) => return None
            };
            if mutex_channel.status == Status::Closed {
                return None;
            }
        }
        return mutex_channel.channel.pop_front();
    }

    fn shutdown(&self) -> Option<()> {
        let mut channel = match self.mutex_channel.lock() {
            Ok(res) => res,
            Err(_) => return None
        };
        channel.status = Status::Closed;
        self.cvar.notify_all();
        return Some(());
    }
}

fn main() {
    // Creiamo una nuova istanza di MpMcChannel con un buffer circolare di 10 elementi
    let channel = MpMcChannel::new(10);

    // Creiamo alcuni produttori
    for i in 0..5 {
        let channel = channel.clone(); // Cloniamo il canale per ogni produttore
        thread::spawn(move || {
            for j in 0..5 {
                // Inviamo dati al canale
                channel.send(j).unwrap();
                println!("Produttore {} ha inviato {}", i, j);
            }
        });
    }

    // Creiamo alcuni consumatori
    for i in 0..3 {
        let channel = channel.clone(); // Cloniamo il canale per ogni consumatore
        thread::spawn(move || {
            loop {
                // Riceviamo dati dal canale
                match channel.recv() {
                    Some(data) => println!("Consumatore {} ha ricevuto {}", i, data),
                    None => {
                        println!("Consumatore {} ha terminato.", i);
                        break;
                    }
                }
            }
        });
    }

    // Aspettiamo che tutti i thread terminino
    thread::sleep(std::time::Duration::from_secs(5));

    // Chiudiamo il canale
    channel.shutdown().unwrap();
}
