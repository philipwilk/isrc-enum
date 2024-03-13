use isrc::Isrc;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let code = Isrc::try_from(input);
    match code {
        Ok(res) => {
            println!("you typed the isrc: {:?}", res);
        }
        Err(err) => {
            println!("Err: {}", err);
        }
    }
}
