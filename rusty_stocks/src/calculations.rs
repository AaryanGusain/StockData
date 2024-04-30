use std::intrinsics::floorf32;

use crate::stock::{self, Stock};

use clap::Error;
use rand::seq::SliceRandom;
use randomforest::criterion::Mse;
use randomforest::table::{Table, TableBuilder, TableError};
use randomforest::RandomForestRegressorOptions;

pub fn construct_table(stocks: &Vec<Stock>) -> TableBuilder {
    let mut table_builder: TableBuilder = TableBuilder::new();

    for stock in stocks {
        let _ = table_builder.add_row(&stock.get_array(), stock.get_label());
    }

    table_builder
}

fn split_data(stocks: &Vec<Stock>, training: f32) -> (Vec<Stock>, Vec<Stock>) {
    let mut indices: Vec<usize> = (0..stocks.len()).collect();
    indices.shuffle(&mut rand::thread_rng());
    let training_index: usize = (training * (stocks.len() as f32)) as usize;
    let mut training_set: Vec<Stock> = Vec::new();
    for idx in indices[0..training_index].iter() {
        training_set.push(stocks[*idx]);
    }

    let mut test_set: Vec<Stock> = Vec::new();
    for idx in indices[training_index..].iter() {
        test_set.push(stocks[*idx]);
    }

    (training_set, test_set)
}
