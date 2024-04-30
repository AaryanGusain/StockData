use crate::stock::Stock;
use rand::seq::SliceRandom;
use randomforest::criterion::Mse;
use randomforest::table::{Table, TableBuilder, TableError};
use randomforest::RandomForestRegressorOptions;

/*
    Constructs a randomforest crate TableBuilder which holds the stock data from
    the passed stock struct vector

    @param (stocks: &Vec<Stock>) vector of stock structs containing training dataset

    @return (TableBuilder) TableBuilder object with stock data inserted
*/
pub fn construct_table(stocks: &Vec<Stock>) -> TableBuilder {
    let mut table_builder: TableBuilder = TableBuilder::new();

    for stock in stocks {
        let _ = table_builder.add_row(&stock.get_array(), stock.get_label());
    }

    table_builder
}

/*
    Splits stocks into two sets, training and testing for cross referencing testing

    @param (stocks: &Vec<stock>) vector of stock structs parsed from file
    @param (training: f32) fraction of dataset to be in the training set

    @return (Vec<Stock>, Vec<Stock) partitioned training and testing datasets respectively
*/
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
