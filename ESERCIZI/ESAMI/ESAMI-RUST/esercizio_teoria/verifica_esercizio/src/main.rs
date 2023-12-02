enum Command {
    Add(u8, u8), // tag = 00
    Sub(u16),    // tag = 01
    Or(bool),    // tag = 02
    Clear,       // tag = 03
}

fn main() {
    let v = vec![
        Command::Clear,
        Command::Add(3, 2),
        Command::Sub(10),
        Command::Or(true),
        Command::Clear,
        Command::Add(3, 2),
        Command::Sub(10),
        Command::Or(true),
    ];
    let _slice: &[Command] = &v[..];
    let _breakpoint = 0;
}
