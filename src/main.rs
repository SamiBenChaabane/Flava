mod generation;
use ansi_term::Colour::Fixed;
use generation::password_gen;
fn main() {
    let mut table_width: u8 = 0;

    print!("╭");
    while table_width != 14 {
        print!("─");
        table_width += 1;
    }

    print!("╮\n│ {} │\n╰", Fixed(41).paint(password_gen()));

    table_width = 0;

    while table_width != 14 {
        print!("─");
        table_width += 1;
    }
    println!("╯");
}
