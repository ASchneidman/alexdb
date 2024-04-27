use std::{fmt::format, io::{self, Write}};

enum StatementType {
    INSERT,
    SELECT
}

struct Statement {
    typ: StatementType,
}

fn print_prompt() {
    print!("alexdb > ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn prepare_statement(line: &str) -> Result<Statement, String> {
    if line.starts_with("INSERT ") {
        return Ok(Statement {
                typ: StatementType::INSERT,
            });
    } else if (line.starts_with("SELECT ")) {
        return Ok(Statement {
            typ: StatementType::SELECT,
        });
    }
    return Err(format!("Unrecognized statement {line}"));
}

fn execute_statement(statement: Statement) -> Result<(), String> {
    match statement.typ {
        StatementType::INSERT => println!("Would execute INSERT"),
        StatementType::SELECT => println!("Would execute SELECT"),
    }
    return Ok(());
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
            l => match prepare_statement(l) {
                Ok(statement) => match execute_statement(statement) {
                    Ok(()) => println!("Executed."),
                    Err(msg) => println!("Failed to execute statement: {}", msg),
                },
                Err(msg) => println!("Failed to prepare statement: {}", msg),
            },
        }
        line.clear();
    }
}
