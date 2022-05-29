use ansi_term::Colour::Cyan;
pub fn display_password(mut password: String) {
    let mut table_width: usize = password.len();
    if password.len() > 80 {
        //This section displays passwords that cannot fit on a single line, meaning >= 80 characters
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
        //This section displays passwords that can fit on a single line, meaning <= 80 characters
        print!("{}", Cyan.paint(format!("╭{:─^table_width$}╮\n", "")));
        print!(
            "{}{password}{}",
            Cyan.paint(format!("│")),
            Cyan.paint(format!("│\n"))
        );
        print!("{}", Cyan.paint(format!("╰{:─^table_width$}╯\n", "")));
    }
}
pub fn display_analysis() {}
