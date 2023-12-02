
/*
Un centro ricreativo dispone di un numero limitato di sale per attività. Il tuo compito è implementare un
sistema di gestione delle prenotazioni che consenta a più utenti di prenotare e liberare le sale in modo concorrente.
Dovrai garantire che le prenotazioni non entrino in conflitto e che le risorse siano allocate in modo corretto.
Considera che ogni utente può prenotare una sala solo per un certo periodo di tempo.

Devi implementare le seguenti funzionalità:

    Prenotazione: Gli utenti possono prenotare una sala per un certo periodo di tempo. Se la sala è già prenotata in
        quel momento, l'utente deve attendere fino a quando non diventa disponibile.

    Liberazione: Gli utenti possono liberare una sala dopo averla utilizzata.

    Verifica disponibilità: Gli utenti possono verificare se una sala è disponibile in un certo intervallo di tempo.

Implementa una struttura dati BookingSystem con i seguenti metodi:

    new(num_rooms: usize) -> Self: Crea un nuovo sistema di prenotazioni con il numero specificato di sale.

    reserve(&self, room_id: usize, start_time: u32, end_time: u32) -> bool: Prenota una sala specifica per il periodo
        specificato. Restituisce true se la prenotazione è riuscita, false se la sala è già prenotata.

    release(&self, room_id: usize, start_time: u32, end_time: u32) -> bool: Libera una sala specifica
        che è stata prenotata precedentemente.

    check_availability(&self, room_id: usize, start_time: u32, end_time: u32) -> bool: Verifica se una
        sala è disponibile nel periodo specificato.

Assicurati che la tua implementazione gestisca correttamente la concorrenza tra le operazioni di prenotazione e liberazione delle sale.
*/

use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

struct Room {
    room_id: usize,
    start_time: usize,
    end_time: usize
}

pub struct BookingSystem {
    rooms: Mutex<Vec<Room>>,
    num_rooms: usize,
}

impl BookingSystem {
    pub fn new(num_rooms: usize) -> Self {
        Self {
            rooms: Mutex::new(Vec::with_capacity(num_rooms)),
            num_rooms
        }
    }

    pub fn reserve(&self, room_id: usize, start_time: usize, end_time: usize) -> bool {
        if end_time <= start_time {
            return false;
        }
        let mut rooms = self.rooms.lock().unwrap();
        let mut availability = Self::check_availability_private(&rooms, room_id, start_time, end_time);
        if availability {
            rooms.push(Room {room_id, start_time, end_time});
        }

        return availability;
    }

    pub fn release(&self, room_id:usize, start_time: usize, end_time:usize) -> bool{
        let mut rooms = self.rooms.lock().unwrap();
        let mut pos = self.num_rooms+1;
        for i in 0..rooms.len() {
            if rooms[i].room_id == room_id && rooms[i].start_time == start_time && rooms[i].end_time == end_time {
                pos = i;
            break;
            }
        }
        if pos != self.num_rooms+1 {
            rooms.remove(pos);
        }

        return pos != self.num_rooms+1;
    }

    pub fn check_availability(&self, room_id: usize, start_time: usize, end_time: usize) -> bool {
        let rooms = self.rooms.lock().unwrap();
        return Self::check_availability_private(&rooms, room_id, start_time, end_time);
    }

    fn check_availability_private(rooms: &MutexGuard<Vec<Room>>, room_id: usize, start_time: usize, end_time: usize) -> bool {
        let mut availability = true;
        for room in rooms.iter().filter(|r| r.room_id == room_id) {
            if !(start_time > room.end_time || (start_time < room.start_time && end_time < room.end_time)) {
                availability = false;
                break;
            }
        }
        return availability;
    }
}







fn main() {
        let booking_system = Arc::new(BookingSystem::new(5)); // 5 rooms

        let num_threads = 10;
        let mut handles = vec![];

        for i in 0..num_threads {
            let booking_system_clone = booking_system.clone();
            let handle = thread::spawn(move || {
                // Simulate reservation and release operations
                let room_id = i % 5; // Each thread selects a room
                let start_time = i * 2;
                let end_time = start_time + 3;
                if booking_system_clone.reserve(room_id, start_time, end_time) {
                    println!("Thread {} reserved room {} from {} to {}", i, room_id, start_time, end_time);
                    thread::sleep(std::time::Duration::from_secs(2));
                    if booking_system_clone.release(room_id, start_time, end_time) {
                        println!("Thread {} released room {} from {} to {}", i, room_id, start_time, end_time);
                    }
                } else {
                    println!("Thread {} could not reserve room {} from {} to {}", i, room_id, start_time, end_time);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
}
