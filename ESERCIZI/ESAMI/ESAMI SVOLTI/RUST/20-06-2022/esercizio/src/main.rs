use std::cell::{Cell, RefCell};
use std::ptr::null;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(PartialEq, Clone)]
enum Status {
    Reading, // leggi il messaggio
    Exit,    // termina l'esecuzione
}

struct Looper<Message> {
    channel: Arc<Mutex<Vec<(Message, Status)>>>,
    cvar: Arc<Condvar>,
    thread: JoinHandle<()>,
}

impl<Message> Looper<Message>
    where
        Message: Send + Clone + 'static,
{
    fn new<P, FC>(process: P, cleanup: FC) -> Self
        where
            P: Fn(Message) + Send + 'static,
            FC: FnOnce() + Send + 'static,
    {
        let channel = Arc::new(Mutex::new(Vec::<(Message, Status)>::new()));
        let cvar = Arc::new(Condvar::new());

        let channel_cloned = channel.clone();
        let cvar_cloned = cvar.clone();
        let thread = thread::spawn(move || loop {
            let mut channel = channel_cloned.lock().unwrap();
            channel = cvar_cloned
                .wait_while(channel, |value| value.len() == 0)
                .unwrap();

            for i in 0..channel.len() {
                if channel[i].1 == Status::Exit {
                    cleanup();
                    return;
                } else {
                    process(channel[i].0.clone());
                }
            }
        });

        Self {
            channel: channel.clone(),
            cvar: cvar.clone(),
            thread,
        }
    }

    fn send(&self, message: Message) {
        let mut channel = self.channel.lock().unwrap();
        channel.push((message, Status::Reading));
        self.cvar.notify_one();
    }
}

/*impl<Message> Drop for Looper<Message> {
    fn drop(&mut self) {
        let mut channel = self.channel.lock().unwrap();
        channel.push((null(), Status::Exit));
    }
}*/

use std::mem;
use std::rc::Rc;

#[repr(C)]
enum AsVector {
    AsVector(Box<Rc<i32>>),
    None
}

#[repr(C)]
struct Data {
    element: AsVector,
    next: Rc<Data>
}

/*
    ALLINEAMENTO

    i8 : nessun allineamento
    i16: 2 byte allineamento
    i32: 4 byte allineamento
    i64: 8 byte allineamento
    i128: 8 byte allineamento

    struct MyStruct {
        field1: u8,     1B
        field2: u32,    4B
        field3: u16,    2B
    }
    allineamento per 4B (più grande)
    dimensione totale con allineamento = 1B+3B (3B di padding) + 4B + 2B+2B (2B di padding) = 4B * 3 = 12B
    dimensione totale con allineamento con ottimizzazione = u8, u16, u32 = 1B+2B+1B (1B di padding) + 4B = 8B

    struct MyStruct {
        field1: u8,     1B
        field2: u128,   16B
        field3: u16,    2B
    }
    allineamento per 8B (più grande)
    dimensione totale con allineamento = 1B+7B + 8B+8B + 2B+6B = 8B * 4 = 32B  (u128 sono 16B ma divisi in 2 da 8B)
    dimensione totale con allineamento con ottimizzazione = u8, u16, u128 = 1B+2B+5B + 8B+8B = 8B * 3 = 24B

    enum MyEnum {
        Variant1(Rc<u32>),              8B
        Variant2(RefCell<Box<u64>>),    8B+8B=16B
    }
    allineamento per 8B (sarebbe 16B ma il massimo possibile su un architettura a 64bit sono 8B altrimenti su un architettura da 32bit sarebbero 4B)
    dimensione totale con allineamento = 1B+7B (1B di TAG e 7B di padding) + 8B+8B = 8B * 3 = 24B
    24B è la dimensione dell'enum in qualsiasi caso perchè lo abbiamo calcolato per il caso più grande (peggiore),
    nel caso della Variant1 sarebbero 1B+7B (1B di TAG e 7B di padding) + 8B (puntatore RC) + 8B (padding)

    enum MyEnum {
        Variant2(u32, u16),
        Variant3(u8),
    }
    allineamento per 4B
    dimensione totale con allineamento = 1B+3B(1B di TAG e 3B di padding) + 4B(u32) + 2B+2B(u16+2B di padding) = 4B * 3 = 12B
    dimensione totale con allineamento con ottimizzazione = 1B+2B+1B(1B di TAG, 2B di u16, 1B di padding) + 4B = 8B


    enum AsVector {
        AsVector(Box<Rc<i32>>),     // 1B+7B + 8B
        None                        // 1B+7B+8B
    }
    struct Data {
        element: AsVector,          // 8B+8B = 16B
        next: Rc<Data>              // 8B
    }                               // allineamento a 8B, dimensione totale = 16B + 8B = 24B

    heap enum = 8B * 4 = 32B
        8B per il puntatore del Rc
        8B (strong) + 8B (weak) + 4B+4B (i32+padding)

    heap struct = 8B + 8B + 24B = 40B
        8B (strong) + 8B (weak) + 24B

 */

fn main() {
    /*let looper = Looper::new(|message: &str| println!("message: {}", message), || println!("Exit"));

    looper.send("Hi");
    looper.send("Ciao");
    looper.send("How");
    looper.send("Are");
    looper.send("You");

    drop(looper);*/

    println!("Dimensione dell'enum: {} byte", mem::size_of::<AsVector>());
    println!("Allineamento dell'enum: {} byte", mem::align_of::<AsVector>());

}
