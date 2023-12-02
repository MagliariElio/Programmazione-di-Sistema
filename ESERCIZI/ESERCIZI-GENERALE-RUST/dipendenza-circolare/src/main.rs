use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Nodo {
    valore: i32,
    successivo: Option<Rc<RefCell<Nodo>>>,
}

fn main() {
    let primo = Rc::new(RefCell::new(Nodo {
        valore: 1,
        successivo: None,
    }));

    let secondo = Rc::new(RefCell::new(Nodo {
        valore: 2,
        successivo: Some(Rc::clone(&primo)),
    }));

    // Aggiorniamo il riferimento al nodo successivo nel primo nodo
    primo.borrow_mut().successivo = Some(Rc::clone(&secondo));

    println!("{:?}", primo);
    println!("{:?}", secondo);
}

/*
fn main_versione_corretta() {
    let primo = Rc::new(RefCell::new(Nodo {
        valore: 1,
        successivo: None,
    }));

    let secondo = Rc::new(RefCell::new(Nodo {
        valore: 2,
        successivo: Some(Rc::downgrade(&primo)),
    }));

    // Aggiorniamo il riferimento al nodo successivo nel primo nodo
    primo.borrow_mut().successivo = Some(Rc::downgrade(&secondo));

    println!("{:?}", primo);
    println!("{:?}", secondo);
}
*/