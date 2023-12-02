
/*
: Scambio di Messaggi tra Thread

Devi implementare una struttura dati in Rust che consenta a due thread di scambiare messaggi tra loro utilizzando un canale.

    Deve essere possibile creare un'istanza di questa struttura dati.

    Un thread deve essere in grado di inviare un messaggio all'altro thread attraverso il canale.

    L'altro thread dovrebbe essere in grado di ricevere il messaggio dal canale.

    Il canale deve essere thread-safe, il che significa che più thread dovrebbero essere in grado di inviare e
        ricevere messaggi contemporaneamente senza errori o condizioni di gara.

    Deve essere possibile chiudere il canale quando non è più necessario.

    Quando il canale viene chiuso, qualsiasi tentativo di inviare un messaggio dovrebbe restituire un errore appropriato.
*/

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender, SendError};

struct ChannelMessage<Message> {
    sender: Sender<Message>,
    receiver: Mutex<Receiver<Message>>
}

impl<Message> ChannelMessage<Message> {
    fn new() -> Arc<Self> {
        (sender, receiver) = channel();
        Arc::new(Self {
            sender,
            receiver: Mutex::new(receiver)
        })
    }

    fn send(&self, message: Message) -> Result<(), SendError<Message>> {
        let sender = self.sender.clone();
        sender.send(message)
    }

    fn recv(&self) -> Result<Message, RecvError> {
        let receiver = self.receiver.lock().unwrap();
        receiver.recv()
    }

    fn close(&mut self) {
        self.sender.cl
    }
}


fn main() {
    println!("Hello, world!");
}
