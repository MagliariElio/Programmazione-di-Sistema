use std::cell::Cell;

fn main() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;

    // Modifichiamo il valore di x senza un riferimento mutabile
    x.set(2);

    // Accediamo al valore di x tramite due riferimenti immutabili y e z
    println!("x = {}, y = {}, z = {}", x.get(), y.get(), z.get());
}
