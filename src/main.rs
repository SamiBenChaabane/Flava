mod analysis;
mod display;
mod generation;

use display::display_analysis;
use display::display_password;
use generation::password_gen;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => display_password(password_gen(&16)),
        2 => match args[1].as_str() {
            "-c" => {
                let mut password = String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("Invalid character(s)");
                if password.trim().is_empty() {
                    println!("No password typed.");
                } else {
                    display_analysis(password.trim());
                }
            }
            "-n" => println!("{}", password_gen(&16)),
            _ => println!("Invalid Arguments"),
        },
        3 => match args[1].as_str() {
            "-l" => match args[2].parse::<usize>() {
                Ok(v) => display_password(password_gen(&v)),
                _ => println!("Invalid Arguments"),
            },
            "-ln" => match args[2].parse::<usize>() {
                Ok(v) => println!("{}", password_gen(&v)),
                _ => println!("Invalid Arguments"),
            },
            _ => println!("Invalid Arguments"),
        },
        _ => println!("Invalid Arguments"),
    }
}
