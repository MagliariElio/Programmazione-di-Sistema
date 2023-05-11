
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if (&code[0..1] == " ") & (&code[1..].chars().count() <= &(1 as usize)) {
        return false;
    }

    if code.len() <= 1 {
        return false;
    }

    let mut tot = 0;
    let mut i = 2;
    let mut number;
    for s in code.chars() {
        if s == ' ' {continue;}

        match s.to_digit(10) {
            Some(valore)  => number = valore * i,
            None => continue
        }

        if number > 9 {
            number -= 9;
        }

        if i <= 1 {
            i = 2;
        } else {
            i = 1;
        }

        tot += number;
        print!("{}\t", number);
    }

    println!("\n{}", tot);
    tot % 10 == 0
}

