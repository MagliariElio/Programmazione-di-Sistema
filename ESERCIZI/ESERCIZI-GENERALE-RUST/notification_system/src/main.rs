use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/*
In un'applicazione concorrente, devi implementare un sistema di notifiche che consenta ai vari componenti
di registrarsi per ricevere notifiche da altri componenti quando avvengono determinati eventi.

Devi creare una struttura dati chiamata NotificationSystem che fornisca i seguenti metodi:
    - new() -> Self: Crea una nuova istanza del sistema di notifiche.
    - register(&self, component_id: usize, event_type: EventType, callback: Box<dyn Fn(Event) + Send + Sync>): Permette a un componente
        di registrarsi per ricevere notifiche di un certo tipo di evento. Il componente viene identificato da un ID univoco (component_id). Il
        callback fornito verrà chiamato quando si verifica l'evento specificato.
    - unregister(&self, component_id: usize): Rimuove la registrazione di un componente dal sistema.
    - notify(&self, event: Event): Notifica tutti i componenti registrati che si è verificato un certo evento. Vengono
        chiamati i callback corrispondenti ai componenti registrati per quel tipo di evento.

La struttura dati NotificationSystem deve essere thread-safe, consentendo la registrazione, la rimozione e la notifica dei componenti in modo concorrente.
Nota: Dovrai definire le strutture EventType e Event in base alle tue esigenze e implementare i metodi nel modo appropriato,
assicurandoti che le operazioni siano thread-safe. Questo problema richiede la gestione delle registrazioni, delle
notifiche e della sincronizzazione tra thread. Può essere un buon esercizio per comprendere come implementare un
sistema di comunicazione tra componenti concorrenti.
*/

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum EventType {
    UserLoggedIn,
    NewMessage,
    TaskCompleted,
}

#[derive(Debug, Clone)]
struct Event {
    event_type: EventType,
    data: String,
}

struct Notification {
    component_id: usize,
    callback: Arc<dyn Fn(Event) + Send + Sync + 'static>,
}

struct NotificationSystem {
    list: Mutex<HashMap<EventType, Vec<Notification>>>,
}

impl NotificationSystem {
    fn new() -> Self {
        Self {
            list: Mutex::new(HashMap::new()),
        }
    }

    fn register(
        &self,
        component_id: usize,
        event_type: EventType,
        callback: Arc<dyn Fn(Event) + Send + Sync + 'static>,
    ) {
        let mut list = self.list.lock().unwrap();
        if let Some(value) = list.get_mut(&event_type) {
            value.push(Notification {
                component_id,
                callback,
            });
        } else {
            let mut v = Vec::new();
            v.push(Notification {
                component_id,
                callback,
            });
            list.insert(event_type, v);
        }
    }

    fn unregister(&self, component_id: usize) {
        let mut list = self.list.lock().unwrap();
        for (_, v) in list.iter_mut() {
            for i in 0..v.len() {
                if v[i].component_id == component_id {
                    v.remove(i);
                }
            }
        }
    }

    fn notify(&self, event: Event) {
        let list = self.list.lock().unwrap();
        if let Some(v) = list.get(&event.event_type) {
            for notification in v {
                (notification.callback)(event.clone());
            }
        }
    }
}

fn main() {
    let system = Arc::new(NotificationSystem::new());
    println!();
    let callback_1 = Arc::new(|e: Event| {
        println!(
            "callback function called with the event {:?} with data {}",
            e.event_type, e.data
        )
    }) as Arc<dyn Fn(Event) + Send + Sync + 'static>;
    let callback_2 = Arc::new(|e: Event| {
        println!(
            "callback function called with the event {:?} with data {}",
            e.event_type, e.data
        )
    }) as Arc<dyn Fn(Event) + Send + Sync + 'static>;
    let callback_3 = Arc::new(|e: Event| {
        println!(
            "callback function called with the event {:?} with data {}",
            e.event_type, e.data
        )
    }) as Arc<dyn Fn(Event) + Send + Sync + 'static>;

    system.register(0, EventType::NewMessage, callback_1);
    system.register(1, EventType::TaskCompleted, callback_2);
    system.register(2, EventType::UserLoggedIn, callback_3);

    system.notify(Event {
        event_type: EventType::NewMessage,
        data: "Hello".to_string(),
    });
    system.unregister(0);
    system.notify(Event {
        event_type: EventType::NewMessage,
        data: "Hi".to_string(),
    });
    system.notify(Event {
        event_type: EventType::UserLoggedIn,
        data: "Ciao".to_string(),
    });
    system.notify(Event {
        event_type: EventType::TaskCompleted,
        data: "Hola".to_string(),
    });
}
