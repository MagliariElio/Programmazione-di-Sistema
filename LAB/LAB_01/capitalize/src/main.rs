use main_conv::conversion;

fn main() {
    let user = "questa è una frase".to_string();
    conversion(user);
}

pub mod main_conv {
    pub fn conversion(user: String) -> String {
        // questo per interagire con l'utente da console
        //std::io::stdout().write("Valore: ".as_bytes()).unwrap();
        /*print!("Valore: ");
        stdout().flush().unwrap();

        let stdin = io::stdin();
        match stdin.read_line(&mut user) {
            Ok(..) => {
                print!("valore catturato: {user}");
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }*/

        println!("\nValore catturato: {user}");

        let vec = user.split(" ");
        let mut res = String::new();
        for s in vec {
            res = res + capitalize(s).as_str() + " ";
        }
        if !res.is_empty() {
            res.remove(res.len() - 1);
        }

        println!("Valore aggiornato: {res}");

        return res;
    }

    fn capitalize(s: &str) -> String {
        let mut str = s.to_string();
        let len = str.chars().count();

        if len == 1 {
            str = str.to_uppercase();
        } else if len > 1 {
            str = (&s[0..1]).parse().unwrap();
            str = str.to_uppercase();
            str = str + &s[1..];
        }

        return str;
    }
}
