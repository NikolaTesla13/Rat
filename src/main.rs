/*
    Rat  Copyright (C) 2021  Asandei Stefan Alexandru
    This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `show c' for details.
*/

mod config;
mod crypto;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configs: config::Config = config::Config::new(&args).unwrap_or_else(|err| {
            print!("Problem with parsing the arguments: {}", err);
            process::exit(1);
        });
    
    if configs.mode == "encrypt" {
        println!("Started encrypting, this will take a few seconds.");
        crypto::encrypt_file(&configs.input_file.clone(), &configs.output_file.clone(), &configs.password.clone());
        println!("Successfully encrypted the file {}!", configs.output_file)
    } else if configs.mode == "decrypt" {
        println!("Started decrypting, this will take a few seconds.");
        let content = crypto::decrypt_file(&configs.output_file.clone(), &configs.password.clone());
        println!("Contents of the encrypted file: \n{}", content)  
    }
}
