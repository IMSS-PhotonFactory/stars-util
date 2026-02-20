use rand::Rng;
use std::char::from_u32;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::ExitCode;
use std::{env, io};

const KEY_MIN: u32 = 10;
const KEY_MAX: u32 = 18;
const KEY_COUNT: u32 = 400;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let fname: String;

    match args.len() {
        1 => {
            let mut input = String::new();
            println!("Please enter key filename:");
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("Bye");
                    return ExitCode::SUCCESS;
                }
                Ok(_n) => {
                    fname = check_file_name(input.trim());
                }
                Err(_) => {
                    eprintln!("Error: Reading input!");
                    return ExitCode::FAILURE;
                }
            }
        }
        2 => {
            fname = check_file_name(&args[1]);
        }
        _ => {
            eprintln!("Usage: {} Filename", args[0]);
            return ExitCode::FAILURE;
        }
    }

    if check_file_exists(&fname) {
        println!("{} alredy exists!", fname);
        return ExitCode::FAILURE;
    }
    create_key_file(&fname).expect("error writing key file");
    ExitCode::SUCCESS
}

fn check_file_name(filename: &str) -> String {
    if filename.contains(".key") {
        filename.to_string()
    } else {
        filename.to_string() + ".key"
    }
}

fn check_file_exists(filename: &str) -> bool {
    let file_path = Path::new(filename);
    file_path.exists()
}

fn create_key_file(filename: &str) -> Result<(), std::io::Error> {
    println!("Create key > {filename} ({KEY_COUNT} keys)");
    let mut file = File::create(filename)?;
    let mut rng = rand::rng();
    for _ in 0..KEY_COUNT {
        let klen = rng.random_range(KEY_MIN..KEY_MAX);
        for _ in 0..klen {
            let mut rnum: u32 = rng.random_range(0x21..0x7d);
            if rnum >= 0x60 {
                rnum += 1;
            }
            let c = from_u32(rnum).unwrap();
            write!(file, "{c}")?;
        }
        file.write_all(b"\n")?;
    }
    println!("Done");

    Ok(())
}
