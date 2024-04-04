use clap::{parser::ValueSource, Arg, Command};
use std::error::Error;

type CustomResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    tickers: Vec<String>,
    days: Option<usize>,
}

pub fn get_args() -> CustomResult<Config> {
    let mut matches = Command::new("rusty_stocks")
        .version("0.1.0")
        .author("Derek Warner <derekw3@illinois.edu>, Chengxun Ren <cren8@illinois.edu>, Haozhe Chen <haozhe6@illinois.edu>, Aaryan Singh Gusain <agusain2@illinois.edu>")
        .about("A CLI stock prediction application")
        .arg(
            Arg::new("tickers")
                .help("Stock Ticker(s)")
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

    Ok(())
}
