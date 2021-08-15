/*
    Rat  Copyright (C) 2021  Asandei Stefan Alexandru
    This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `show c' for details.
*/

mod config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configs: config::Config = config::Config::new(&args).unwrap_or_else(|err| {
            print!("Problem with parsing the arguments: {}", err);
            process::exit(1);
        });
    println!("{}, {}, {}", configs.mode, configs.input_file, configs.password);
}
