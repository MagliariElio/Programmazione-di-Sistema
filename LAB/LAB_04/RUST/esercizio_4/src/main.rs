use parallel_letter_frequency::frequency;

fn main() {
    let input = vec!["efx", "ErikSchierboom", "etrepum", "glennpratt", "IanWhitney"];

    println!("{:?}", frequency(&input, 8));
}
