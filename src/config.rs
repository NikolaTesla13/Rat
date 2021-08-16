/*
    Rat  Copyright (C) 2021  Asandei Stefan Alexandru
    This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `show c' for details.
*/

pub struct Config {
    pub mode: String,
    pub input_file: String,
    pub output_file: String,
    pub password: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() == 1 {
            return Err("usage: rat <encrypt|decrypt> file --password <password>");
        }

        let mut mode: String = "".to_string();
        let mut input_file: String = "".to_string();
        let mut password: String = "".to_string();
        let mut coming_stage: u8 =  0;

        for arg in args.iter() {
            if arg.starts_with("--") {
                match arg.as_str() {
                    "--encrypt" => {
                        mode = "encrypt".to_string();
                        coming_stage = 1;
                    },
                    "--decrypt" => {
                        mode = "decrypt".to_string();
                        coming_stage = 1;
                    }
                    "--password" => {
                        coming_stage = 2;
                    },
                    _ => return Err("usage: rat <encrypt|decrypt> file --password <password>"),
                }
            } else {
                if coming_stage == 1 {
                    input_file = arg.clone();
                    coming_stage = 0;
                } else if coming_stage == 2 {
                    password = arg.clone();
                    coming_stage = 0;
                }
            }
        }

        let mut output_file: String = ".".to_owned();
        output_file.push_str(input_file.as_str());

        return Ok(Config {mode, input_file, output_file, password});
    }
}