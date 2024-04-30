use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{parser::ValueSource, Arg, Command};

use crate::calculations::run_forest;
use crate::stock::Stock;
use crate::stock::Tomorrow;

pub mod calculations;
pub mod stock;

type CustomResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    days: usize,
}

/*
    Attempt to open passed files and then parse them into stock objects, passing it to the desired method of prediction

    @param (config: Config) config object constructed by the get_args function

    @return (CustomResult()) custom result object which indicates that the function has finished
*/
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
                    let line: String = line.unwrap_or_else(|_| String::from(""));

                    if line.is_empty() {
                        continue;
                    } else {
                        let line_vec: Vec<&str> = line.split(',').collect();
                        let stock: Stock = Stock::new(
                            String::from(line_vec[0]),
                            line_vec[1].parse().unwrap(),
                            line_vec[2].parse().unwrap(),
                            line_vec[3].parse().unwrap(),
                            line_vec[4].parse().unwrap(),
                            line_vec[5].parse().unwrap(),
                            line_vec[6].parse().unwrap(),
                            Tomorrow::Predict,
                        );
                        stock_vec.push(stock);
                    }
                }

                let length = stock_vec.len();
                for i in 0..(length - 1) {
                    if stock_vec[i].get_close() <= stock_vec[i + 1].get_close() {
                        stock_vec[i].set_tomorrow(Tomorrow::Increase);
                    } else {
                        stock_vec[i].set_tomorrow(Tomorrow::Decrease);
                    }
                }

                let (result, accuracy) = run_forest(stock_vec);

                if result == 1.0 {
                    println!(
                        "The stock is predicted to increase with an accuracy of {}!",
                        accuracy
                    );
                } else {
                    println!(
                        "The stock is predicted to decrease with an accuracy of {}!",
                        accuracy
                    );
                }
            }
        }
    }

    Ok(())
}

/*
    Parses an int from a string slice, used for parsing the number of days

    @param (value: &str) string slice containing the string to be parsed

    @return (CustomResult<usize>) custom result object which can contain the usize result of parsing the string
*/
pub fn parse_int(value: &str) -> CustomResult<usize> {
    match value.parse() {
        Ok(number) if number > 0 => Ok(number),
        _ => Err(From::from(value)),
    }
}

/*
    Opens a passed file which is in respect to the current working directory

    @param (filename: &str) relative file path which is used to open the stock data file

    @return (CustomResult<Box<dyn BufRead>>) BufRead object used to read the passed file
*/
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
