use crate::analysis::PasswordReport;
use ansi_term::Colour::Cyan;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;
pub fn display_password(mut password: String) {
    let mut table_width: usize = password.len();
    if password.len() > 80 {
        table_width = 80;
        let mut newline_insert_index: usize = 80;
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
    let mut password_report = PasswordReport {
        email_captures: vec![],
        dates_captures: vec![],
        credit_card_numbers_captures: vec![],
    };
    password_report.password_analysis(password);
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

    if !password_report.email_captures.is_empty()
        || !password_report.email_captures.is_empty()
        || !password_report.email_captures.is_empty()
    {
        sensitive_table_width = 40;
        println!(
            "{}",
            Yellow.paint(format!(
                "╭{:─^sensitive_table_width$}╮",
                "Sensitive Information Found"
            ))
        );
        for caps in password_report.email_captures {
            println!(
                "{}",
                Yellow.paint(format!("│{caps: <sensitive_table_width$}│"))
            );
        }

        println!(
            "{}",
            Yellow.paint(format!("╰{:─^sensitive_table_width$}╯", ""))
        );
    } else {
        println!(
            "{}",
            Cyan.paint(format!("╭{:─^sensitive_table_width$}╮", ""))
        );
        println!(
            "{}{}{}",
            Cyan.paint("│"),
            Green.paint("No Sensitive Information Found"),
            Cyan.paint("│")
        );
        println!(
            "{}",
            Cyan.paint(format!("╰{:─^sensitive_table_width$}╯", ""))
        );
    }
}
