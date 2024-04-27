use std::io::{self, Write};

fn print_prompt() {
    print!("alexdb > ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        print_prompt();
        stdin.read_line(&mut line).expect("Failed to read line");
        match line.as_str() {
            "" | ".exit\n" => {
                return;
            },
            l => println!("Unrecognized command '{}'", l.trim()),
        }
        line.clear();
    }
}
