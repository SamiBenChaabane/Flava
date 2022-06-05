use crate::password_analysis;
use ansi_term::Colour::Cyan;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

pub fn display_password(mut password: String) {
    let mut table_width: usize = password.len();
    if password.len() > 80 {
        //This section displays passwords that cannot fit on a single line, meaning > 80 characters
        table_width = 80;
        let mut newline_insert_index: usize = 80;
        //Insert a newline every 80 chars
        while newline_insert_index < password.len() {
            password.insert(newline_insert_index, '\n');
            newline_insert_index += 81;
        }
        print!("{}", Cyan.paint(format!("{:─^table_width$}\n", "Start")));
        println!("{password}");
        print!("{}", Cyan.paint(format!("{:─^table_width$}\n", "End")));
    } else {
        //This section displays passwords that can fit on a single line, meaning < 80 characters
        print!("{}", Cyan.paint(format!("╭{:─^table_width$}╮\n", "")));
        print!("{}{password}{}", Cyan.paint("│"), Cyan.paint("│\n"));
        print!("{}", Cyan.paint(format!("╰{:─^table_width$}╯\n", "")));
    }
}

pub fn display_analysis(password: &str) {
    let table_width = password.len();
    let mut sensitive_table_width = 30;
    let valid_matches = password_analysis(password);
    if password.len() > 2 {
        println!("{}", Cyan.paint(format!("╭{:─^table_width$}╮", "")));
        println!(
            "{}{password:^table_width$}{}",
            Cyan.paint("│"),
            Cyan.paint("│")
        );
        println!("{}", Cyan.paint(format!("╰{:─^table_width$}╯", " ↓ ")));
    } else {
        println!("{}", Cyan.paint(format!("╭{:─^table_width$}╮", "")));
        print!(
            "{}{password:^table_width$}{}",
            Cyan.paint("│"),
            Cyan.paint("│")
        );
        println!("{}", Cyan.paint(format!("╰{:─^table_width$}╯", "")));
    }

    match valid_matches.len() {
        0 => {
            println!(
                "{}",
                Cyan.paint(format!("╭{:─^sensitive_table_width$}╮", ""))
            );
            println!(
                "{}{:^table_width$}{}",
                Cyan.paint("│"),
                Green.paint("No Sensitive Information Found"),
                Cyan.paint("│")
            );
            println!(
                "{}",
                Cyan.paint(format!("╰{:─^sensitive_table_width$}╯", ""))
            );
        }
        _ => {
            sensitive_table_width = 27;
            println!(
                "{}",
                Cyan.paint(format!("╭{:─^sensitive_table_width$}╮", ""))
            );
            println!(
                "{}{:^table_width$}{}",
                Cyan.paint("│"),
                Yellow.paint("Sensitive Information Found"),
                Cyan.paint("│")
            );
            println!(
                "{}",
                Cyan.paint(format!("╰{:─^sensitive_table_width$}╯", ""))
            );
        }
    }
}
