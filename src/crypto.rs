/*
    Rat  Copyright (C) 2021  Asandei Stefan Alexandru
    This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `show c' for details.
*/
use encryptfile;
use std::fs;
use std::path::Path;

pub fn encrypt_file(input_file: &String, output_file: &String, password: &String) -> () {

    let mut config = encryptfile::Config::new();

    config.input_stream(encryptfile::InputStream::File(input_file.to_owned()))
        .output_stream(encryptfile::OutputStream::File(output_file.to_owned())) 
        .add_output_option(encryptfile::OutputOption::AllowOverwrite)
        .initialization_vector(encryptfile::InitializationVector::GenerateFromRng)
        .password(encryptfile::PasswordType::Text(password.to_owned(), encryptfile::scrypt_defaults()))
        .encrypt();

    let _ = encryptfile::process(&config).map_err(|err| {
        panic!("Error encrypting the file! {:?}", err)
    });

}

pub fn decrypt_file(input_file: &String, password: &String) -> String {
    
    let mut config = encryptfile::Config::new();
    let tmp_file = format!("/tmp/decrtyped_{}", input_file);
    let str_tmp_file: &str = &tmp_file.as_str();
    
    config.input_stream(encryptfile::InputStream::File(input_file.to_owned()))
        .output_stream(encryptfile::OutputStream::File(tmp_file.to_owned()))
        .add_output_option(encryptfile::OutputOption::AllowOverwrite)
        .password(encryptfile::PasswordType::Text(password.to_owned(), encryptfile::PasswordKeyGenMethod::ReadFromFile))
        .decrypt();

    let _ = encryptfile::process(&config).map_err(|err| {
        panic!("Error decrypting the file! {:?}", err)
    });
    
    let contents = fs::read_to_string(str_tmp_file)
        .expect("Something went wrong reading the file");
    fs::remove_file(Path::new(str_tmp_file))
        .expect("Something went wrong deleting the tmp file");

    contents
}