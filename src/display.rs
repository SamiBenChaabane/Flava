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

pub fn display_analysis(password: &String) {
    let table_width = password.len();
    let mut sensitive_table_width = 31;
    let valid_matches = password_analysis(&password);
    print!("{}", Cyan.paint(format!("╭{:─^table_width$}╮\n", "")));
    print!(
        "{}{password:^table_width$}{}",
        Cyan.paint("│"),
        Cyan.paint("│\n")
    );
    print!("{}", Cyan.paint(format!("╰{:─^table_width$}╯\n", " ↓ ")));

    match valid_matches.len() {
        0 => {
            print!(
                "{}",
                Cyan.paint(format!("╭{:─^sensitive_table_width$}╮\n", ""))
            );
            print!(
                "{}{:^table_width$}{}",
                Cyan.paint("│"),
                Green.paint("Sensitive Information Not Found"),
                Cyan.paint("│\n")
            );
            print!(
                "{}",
                Cyan.paint(format!("╰{:─^sensitive_table_width$}╯\n", ""))
            );
        }
        _ => {
            sensitive_table_width = 27;
            print!(
                "{}",
                Cyan.paint(format!("╭{:─^sensitive_table_width$}╮\n", ""))
            );
            print!(
                "{}{:^table_width$}{}",
                Cyan.paint("│"),
                Yellow.paint("Sensitive Information Found"),
                Cyan.paint("│\n")
            );
            print!(
                "{}",
                Cyan.paint(format!("╰{:─^sensitive_table_width$}╯\n", ""))
            );
        }
    }
}
