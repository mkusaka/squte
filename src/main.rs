use std::io;
use std::io::prelude::*;

fn main() {
    println!("db > ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input_str = line.unwrap();
        if input_str == ".exit" {
            println!("ok, finish");
            break;
        } else {
            println!("unrecognized command {}", input_str);
        }
        println!("db > ")
    }
}
