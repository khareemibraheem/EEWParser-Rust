mod parser;
mod reader;
mod regexp;

use crate::parser::decoder;
use crate::reader::{read_content_from_file, read_content_from_stdin};
use crate::regexp::{MATCH_RULES, has_newline, telegram_parser};

use std::env;
use std::process;

fn main() {
    let mut file_path = String::from("./data.txt");
    let mut is_stdin = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-p" => {
                if let Some(p) = args.next() {
                    file_path = p;
                }
            }
            "-s" => {
                is_stdin = true;
            }
            _ => {
            }
        }
    }

    let tele_data = if is_stdin {
        match read_content_from_stdin() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read telegram data: {e}");
                process::exit(1);
            }
        }
    } else {
        match read_content_from_file(&file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to open or read telegram file: {e}");
                process::exit(1);
            }
        }
    };

    let mut tele_data = tele_data;

    if !has_newline(&tele_data) {
        tele_data = telegram_parser(&tele_data, &MATCH_RULES);
    }

    let t = decoder(&tele_data);

    let json = match serde_json::to_string(&t) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to serialize result to JSON: {e}");
            process::exit(1);
        }
    };

    println!("{json}");
}
