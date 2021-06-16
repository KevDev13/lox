use clap::{ Arg, App };
use std::io;
use std::io::Read;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    //println!("Lox interpreter!");
    let args = App::new("Lox Interpreter")
        .version("0.1")
        .author("Kevin M. Garner <mail@kevingarner.net>")
        .arg(Arg::with_name("FILE")
             .help("Input file to use")
             .required(true)
             .index(1))
        .get_matches();

    let file_name = args.value_of("FILE").unwrap();
    println!("interpreting file {}", file_name);


}

fn run_file(path: String) {
    let mut data = Vec::new();
    let mut f = File::open(path).expect("error opening file");
    f.read_to_end(&mut data).expect("error reading file");
}