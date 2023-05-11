use luhn::is_valid;

fn main() {
    let credit_card = "59%59";                              //"4539 3195 0343 6467";
    //println!("{}", is_valid(credit_card));
    is_valid(credit_card);
}