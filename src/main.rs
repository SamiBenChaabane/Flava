mod display;
mod generation;
use display::display_password;
use generation::password_gen;
use std::env;

fn main() {
    let default: usize = 16;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        display_password(password_gen(&default));
    } else {
        let password_length: usize = args[1].parse::<usize>().unwrap();
        display_password(password_gen(&password_length));
    }
}
