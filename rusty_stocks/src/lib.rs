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
}
#[derive(Debug)]
pub struct Stock {
    date: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    adj_close: f32,
    volume: usize,
}

impl Stock {
    fn get_array(&self) -> [f32; 6] {
        [
            self.open,
            self.high,
            self.low,
            self.close,
            self.adj_close,
            self.volume as f32,
        ]
    }
}

pub fn run(config: Config) -> CustomResult<()> {
    for filename in config.files {
        match open_file(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                println!("{} Successfully Opened! Parsing Data...", filename);

                let mut stock_vec: Vec<Stock> = Vec::new();

                for (line_number, line) in file.lines().enumerate() {
                    if line_number == 0 {
                        continue;
                    }
                    let line: String = match line {
                        Ok(valid_line) => valid_line,
                        Err(_) => String::from(""),
                    };

                    if line.is_empty() {
                        continue;
                    } else {
                        let line_vec: Vec<&str> = line.split(',').collect();
                        let stock: Stock = Stock {
                            date: String::from(line_vec[0]),
                            open: line_vec[1].parse().unwrap(),
                            high: line_vec[2].parse().unwrap(),
                            low: line_vec[3].parse().unwrap(),
                            close: line_vec[4].parse().unwrap(),
                            adj_close: line_vec[5].parse().unwrap(),
                            volume: line_vec[6].parse().unwrap(),
                        };
                        stock_vec.push(stock);
                    }
                }
            }
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
