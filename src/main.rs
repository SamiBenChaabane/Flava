mod analysis;
mod display;
mod generation;
use analysis::password_analysis;
use display::display_analysis;
use display::display_password;
use generation::password_gen;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => display_password(password_gen(&16)),
        2 => match &args[1][0..2] {
            "-c" => {
                let mut password = String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("Invalid character(s)");
                display_analysis(password.trim());
            }
            _ => println!("Invalid Arguments"),
        },
        3 => match &args[1][0..2] {
            "-l" => match args[2].parse::<usize>() {
                Ok(v) => display_password(password_gen(&v)),
                _ => println!("Invalid Arguments"),
            },
            _ => println!("Invalid Arguments"),
        },
        _ => println!("Invalid Arguments"),
    }
}
