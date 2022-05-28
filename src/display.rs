use ansi_term::Colour::Fixed;
pub fn display_password(mut password: String) {
    let mut table_width: usize = password.len();
    if password.len() > 80 {
        //This section displays passwords that cannot fit on a single line, meaning > 80 characters
        table_width = 80;
        let mut newline_insert_index: usize = 80;
        while newline_insert_index < password.len() {
            password.insert(newline_insert_index, '\n');
            newline_insert_index += 81;
        }
        print!(
            "{}",
            Fixed(215).paint(format!("{:─^table_width$}\n", "Start"))
        );
        println!("{}", Fixed(42).paint(password));
        print!(
            "{}",
            Fixed(215).paint(format!("{:─^table_width$}\n", "End"))
        );
    } else {
        //This section displays passwords that can fit on a single line, meaning <=80 characters
        print!("{}", Fixed(215).paint(format!("╭{:─^table_width$}╮\n", "")));
        println!(
            "{}{}{}",
            Fixed(215).paint("│"),
            Fixed(42).paint(password),
            Fixed(215).paint("│")
        );
        print!("{}", Fixed(215).paint(format!("╰{:─^table_width$}╯\n", "")));
    }
}
