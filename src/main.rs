use std::{fmt::format, io::{self, Write}};
use regex::Regex;


enum StatementType {
    INSERT,
    SELECT
}

trait Statement {
    fn execute(&self) -> Result<(), String>;
    fn typ(&self) -> &StatementType;
}

struct Insert {
    typ: StatementType,
    row: Row
}

impl Statement for Insert {
    fn execute(&self) -> Result<(), String> {
        println!("Executed INSERT");
        return Ok(());  
    }
    fn typ(&self) -> &StatementType {
        return &self.typ;
    }
}

struct Select {
    typ: StatementType,
}

impl Statement for Select {
    fn execute(&self) -> Result<(), String> {
        println!("Executed SELECT");
        return Ok(());  
    }
    fn typ(&self) -> &StatementType {
        return &self.typ;
    }
}

struct Row {
    id: u64,
    username: String,
    email: String,
}

fn print_prompt() {
    print!("alexdb > ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn prepare_statement(line: &str) -> Result<Box<dyn Statement>, String> {
    let insert_re = Regex::new(r"(INSERT|insert) ([0-9]+) (\S+) (\S+)").unwrap();
    if line.starts_with("INSERT") {
        let Some(captures) = insert_re.captures(line) else {
          return Err(format!("Invalid INSERT statement: {line}"));   
        };
        let Ok(id) = captures[2].parse() else {
            return Err(format!("Invalid id."));
        };
        return Ok(Box::new(Insert {
                typ: StatementType::INSERT,
                row: Row {
                    id: id,
                    username: captures[3].to_string(),
                    email: captures[4].to_string(),
                },
            }));
    } else if line.starts_with("SELECT") {
        return Ok(Box::new(Select {
            typ: StatementType::SELECT,
        }));
    }
    return Err(format!("Unrecognized statement {line}"));
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
                Ok(statement) => match statement.execute() {
                    Ok(()) => println!("Executed."),
                    Err(msg) => println!("Failed to execute statement: {}", msg),
                },
                Err(msg) => println!("Failed to prepare statement: {}", msg),
            },
        }
        line.clear();
    }
}
