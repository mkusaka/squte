use std::io;
use std::io::prelude::*;

enum MetaCommandResult {
    META_COMMAND_SUCCESS,
    META_COMMAND_UNRECOGNIZED_COMMAND,
}

enum Statement {
    STATEMENT_INSERT,
    STATEMENT_SELECT,
    STATEMENT_UNRECOGNIZED,
}

fn main() {
    println!("db > ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input_str = line.unwrap();
        if input_str.chars().next().unwrap() == '.' {
            // meta commands
            match do_meta_command(input_str.as_str()) {
                MetaCommandResult::META_COMMAND_SUCCESS => {
                    println!("ok, finish");
                    break;
                }
                MetaCommandResult::META_COMMAND_UNRECOGNIZED_COMMAND => {
                    println!("Unrecognized command {}", input_str)
                }
            }
        } else {
            match prepare_staement(input_str.as_str()) {
                Statement::STATEMENT_INSERT => println!("insert"),
                Statement::STATEMENT_SELECT => println!("unrecongnized keyword {}", input_str),
                Statement::STATEMENT_UNRECOGNIZED => {
                    execute_statement(&input_str);
                    println!("executed")
                }
            }
        }
        println!("db > ")
    }
}

fn do_meta_command(statement: &str) -> MetaCommandResult {
    if (statement == ".exit") {
        MetaCommandResult::META_COMMAND_SUCCESS
    } else {
        MetaCommandResult::META_COMMAND_UNRECOGNIZED_COMMAND
    }
}

fn prepare_staement(statement: &str) -> Statement {
    match &((*statement)[..5]) {
        "insert" => Statement::STATEMENT_INSERT,
        "select" => Statement::STATEMENT_SELECT,
        _ => Statement::STATEMENT_UNRECOGNIZED,
    }
}

fn execute_statement(statement: &str) {
    unreachable!()
}
