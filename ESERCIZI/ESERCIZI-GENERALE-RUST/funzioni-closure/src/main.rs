fn main() {
    let x = 10;
    let funzione_fn = || x + 1;
    println!("Risultato della funzione fn: {}", funzione_fn());
    println!("x: {}", x); // x si può ancora riusare

    let mut y = 20;
    let mut funzione_fn_mut = || {
        y += 1;
        y
    };
    println!("\nRisultato della funzione fn_mut: {}", funzione_fn_mut());
    println!("y: {}", y); // y si può ancora riusare

    let z = 30;
    let funzione_fn_once = move || {
        let result = z + 3;
        result
    };
    println!("\nRisultato della funzione fn_once: {}", funzione_fn_once());
    println!("z: {}", z); // z non si può più usare
}
