use clap::{parser::ValueSource, Arg, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type CustomResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    days: usize,
}

pub fn run(config: Config) -> CustomResult<()> {
    for filename in config.files {
        match open_file(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(_) => println!("{} Successfully Opened", filename),
        }
    }

    Ok(())
}

pub fn parse_int(value: &str) -> CustomResult<usize> {
    match value.parse() {
        Ok(number) if number > 0 => Ok(number),
        _ => Err(From::from(value)),
    }
}

fn open_file(filename: &str) -> CustomResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> CustomResult<Config> {
    let mut matches = Command::new("rusty_stocks")
        .version("0.1.0")
        .author("Derek Warner <derekw3@illinois.edu>, Chengxun Ren <cren8@illinois.edu>, Haozhe Chen <haozhe6@illinois.edu>, Aaryan Singh Gusain <agusain2@illinois.edu>")
        .about("A CLI stock prediction application")
        .arg(
            Arg::new("files")
                .help("Input File(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_days")
                .short('n')
                .long("days")
                .value_name("DAYS")
                .help("Number of days to predict")
                .num_args(1)
                .default_value("10"),   
        )
        .get_matches();

    let files_vec: Vec<String> = matches.remove_many("files").unwrap().collect();

    let number_days_flag = matches!(
        matches.value_source("number_days").unwrap(),
        ValueSource::CommandLine
    );

    let mut number_days: usize = 10;
    if number_days_flag {
        let input_string: String = matches.remove_one("number_days").unwrap();
        let input_number_days = parse_int(&input_string);
        match input_number_days {
            Ok(number) => number_days = number,
            Err(e) => return Err(e).map_err(|e| format!("invalid day count -- {}", e))?,
        }
    }

    Ok(Config {
        files: files_vec,
        days: number_days,
    })
}
