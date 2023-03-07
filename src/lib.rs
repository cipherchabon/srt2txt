use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use clap::{Arg, Command};
use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    input: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("srt2txt")
        .version("0.0.1")
        .author("Marcos Alejandro <cypherchabon@gmail.com")
        .about("Converts SRT subtitles to plain text")
        .arg(
            Arg::new("input")
                .required(true)
                .help("Sets the input file to use"),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();

    Ok(Config {
        input: input_path.to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // First check is the input file extension is .srt and if not, return an error
    if !config.input.ends_with(".srt") {
        return Err("Input file must have .srt extension".into());
    }

    let path = std::path::Path::new(&config.input);

    // Second check is the input file exists and if not, return an error
    if !path.exists() {
        return Err("Input file does not exist".into());
    }

    // Third check is the input file is readable and if not, return an error
    if !path.is_file() {
        return Err("Input file is not a file".into());
    }

    // Abrir el archivo
    let input_file = File::open(path)?;
    let reader = BufReader::new(input_file);

    // Procesar cada línea del archivo
    let mut output_string = String::new();
    let mut current_text = String::new();

    for line in reader.lines() {
        let line_content = line?;

        // If the line is a number, ignore it
        if line_content.chars().all(|c| c.is_numeric()) {
            continue;
        }

        // If the line is a time marker, ignore it
        let time_regex =
            Regex::new(r"\d{2}:\d{2}:\d{2},\d{3}\s*-->\s*\d{2}:\d{2}:\d{2},\d{3}").unwrap();
        if time_regex.is_match(&line_content) {
            continue;
        }

        let case_1 = current_text.ends_with(".");
        let case_2 = current_text.ends_with("?");
        let case_3 = current_text.ends_with("!");
        let case_4 = current_text.ends_with("...");
        let case_5 = current_text.ends_with("…");

        let some_case = case_1 || case_2 || case_3 || case_4 || case_5;

        if some_case && line_content.starts_with(char::is_uppercase) {
            current_text.push_str("\n");
        } else {
            current_text.push_str(" ");
        }

        current_text.push_str(&line_content);
    }
    output_string.push_str(&current_text);

    let output_path = path.with_extension("txt");
    let output_file = File::create(output_path)?;
    let mut writer = std::io::BufWriter::new(output_file);
    writer.write_all(output_string.trim().as_bytes())?;

    Ok(())
}
