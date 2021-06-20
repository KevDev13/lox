use clap::{ Arg, App };
use std::io::Read;
use std::str;

mod token;
use token::*;
mod scanner;
use scanner::*;

static mut had_error: bool = false;

fn had_err() -> bool {
    unsafe { had_error }
}

fn declare_error() {
    unsafe {
        had_error = true;
    }
}

fn main() {
    let args = App::new("Lox Interpreter")
        .version("0.1")
        .author("Kevin M. Garner <mail@kevingarner.net>")
        .arg(Arg::with_name("FILE")
             .help("Input file to use")
             .required(false)
             .index(1))
        .get_matches();

    let file_name = args.value_of("FILE").unwrap();
    println!("interpreting file {}", file_name);
    run_file(file_name.to_string());
}

fn run_file(path: String) {
    let data = std::fs::read_to_string(path).expect("Error reading file");

    run(data);

    if had_err() {
        return;
    }
}

fn run(data: String) {
    let mut scanner: Scanner = Scanner::new(data);
    let tokens = scanner.scan_tokens();

    for t in &tokens {
        println!("{}", t);
    }
}

fn error(line: usize, msg: String) {
    report(line, "".to_string(), msg);
}

fn report(line: usize, w: String, msg: String) {
    println!("[{}] Error{}: {}", line, w, msg);
    declare_error();
}
