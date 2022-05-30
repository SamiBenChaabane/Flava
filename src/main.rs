mod analysis;
mod display;
mod generation;

use analysis::password_analysis;
use display::display_password;
use generation::password_gen;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => display_password(password_gen(&16)),
        2 => match args[1].parse::<usize>() {
            Ok(v) => display_password(password_gen(&v)),
            _ => println!("Invalid Arguments"),
        },
        3 => println!("Invalid Arguments"),
        _ => println!("Invalid Arguments"),
    }
}
