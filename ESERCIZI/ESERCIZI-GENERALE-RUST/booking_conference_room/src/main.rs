/*
Hai il compito di implementare un sistema di gestione delle prenotazioni per una sala conferenze. Il
sistema deve essere in grado di gestire le prenotazioni per diverse date e orari. Ogni prenotazione è
associata a un nome (il nome della persona che ha effettuato la prenotazione) e una data e ora specifica.

Devi implementare le seguenti funzionalità utilizzando una struttura dati HashMap:

    new() -> Self: Crea una nuova istanza del sistema di gestione delle prenotazioni.

    prenota(&mut self, data_ora: DateTime<Utc>, nome: String) -> Result<(), String>: Registra una prenotazione
    per la data e l'ora specificate. Se la data e l'ora sono già occupate, restituisci un errore con un messaggio
    appropriato. Altrimenti, registra la prenotazione e restituisci un risultato di successo.

    annulla(&mut self, data_ora: DateTime<Utc>) -> Result<(), String>: Annulla una prenotazione esistente per la
    data e l'ora specificate. Se la data e l'ora non corrispondono a una prenotazione esistente, restituisci un
    errore con un messaggio appropriato. Altrimenti, annulla la prenotazione e restituisci un risultato di successo.

    verifica_disponibilita(&self, data_ora: DateTime<Utc>) -> bool: Verifica se la data e l'ora specificate sono
    disponibili per una nuova prenotazione. Restituisci true se la data e l'ora sono libere, altrimenti false.

    elenco_prenotazioni(&self) -> Vec<(DateTime<Utc>, String)>: Restituisci un elenco di tutte le prenotazioni
    ordinate per data e ora, insieme al nome della persona che ha effettuato la prenotazione.

Assicurati di gestire accuratamente i casi in cui le date e le ore possono sovrapporsi e di fornire messaggi
di errore significativi in caso di problemi.
*/

use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

struct Prenotazioni {
    booking: Mutex<HashMap<Instant, String>>,
}

impl Prenotazioni {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            booking: Mutex::new(HashMap::new()),
        })
    }

    fn prenota(&self, data_ora: Instant, nome: String) -> Result<(), &str> {
        let mut booking = self.booking.lock().unwrap();
        if let Some(result) = booking.get(&data_ora) {
            return Err("Sala conferenza occupate in quel momento");
        }

        booking.insert(data_ora, nome);
        return Ok(());
    }

    fn annulla(&self, data_ora: Instant) -> Result<(), &str> {
        let mut booking = self.booking.lock().unwrap();
        match booking.remove(&data_ora) {
            Some(_) => Ok(()),
            None => Err("Prenotazione non presente a questo orario"),
        }
    }

    fn verifica_disponibilita(&self, data_ora: Instant) -> bool {
        let booking = self.booking.lock().unwrap();
        match booking.get(&data_ora) {
            Some(_) => false,
            None => true,
        }
    }

    fn elenco_prenotazioni(&self) -> Vec<(Instant, String)> {
        let booking = self.booking.lock().unwrap();
        let mut elenco = Vec::with_capacity(booking.len());
        for element in booking.iter() {
            elenco.push((element.0.clone(), element.1.clone()));
        }

        elenco.sort_by(|current, next| current.0.cmp(&next.0));

        return elenco;
    }
}

fn main() {
    let mut gestore_prenotazioni = Prenotazioni::new();

    // Esempi di utilizzo
    let now = Instant::now() + Duration::from_secs(40);
    let one_hour_later = Instant::now() + Duration::from_secs(36);

    gestore_prenotazioni
        .prenota(now, "Alice".to_string())
        .unwrap();

    gestore_prenotazioni
        .prenota(one_hour_later, "Bob".to_string())
        .unwrap();

    // Verifica disponibilità
    let disponibile = gestore_prenotazioni.verifica_disponibilita(now);
    println!("Disponibile: {}", disponibile);

    // Elenco delle prenotazioni
    let elenco = gestore_prenotazioni.elenco_prenotazioni();
    for (data_ora, nome) in elenco {
        println!("Data/Ora: {:?}, Nome: {}", data_ora, nome);
    }
}
