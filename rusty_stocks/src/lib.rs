use clap::{parser::ValueSource, Arg, Command};
use std::error::Error;

type CustomResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    tickers: Vec<String>,
    days: Option<usize>,
}
