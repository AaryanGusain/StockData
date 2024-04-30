use crate::stock::Stock;
use rand::seq::SliceRandom;
use randomforest::criterion::Gini;
use randomforest::table::{Table, TableBuilder};
use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};

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
    Splits stocks into two sets, training and testing for cross reference testing

    @param (stocks: &Vec<stock>) vector of stock structs parsed from file
    @param (training: f32) fraction of dataset to be in the training set

    @return (Vec<Stock>, Vec<Stock) partitioned training and testing datasets respectively
*/
pub fn split_data(stocks: &[Stock], training: f32) -> (Vec<Stock>, Vec<Stock>) {
    let mut indices: Vec<usize> = (0..stocks.len()).collect();
    indices.shuffle(&mut rand::thread_rng());
    let training_index: usize = (training * (stocks.len() as f32)) as usize;
    let mut training_set: Vec<Stock> = Vec::new();
    for idx in indices[0..training_index].iter() {
        training_set.push(stocks[*idx].clone());
    }

    let mut test_set: Vec<Stock> = Vec::new();
    for idx in indices[training_index..].iter() {
        test_set.push(stocks[*idx].clone());
    }

    (training_set, test_set)
}

/*
    Builds the random forest and predicts if it will increase or decrease between today and tomorrow

    @param (stocks: Vec<Stock>) vector of Stock objects parsed from the input file

    @return (f64, f32) the predicted result and accuracy respectively
*/
pub fn run_forest(stocks: Vec<Stock>) -> (f64, f32) {
    let ultimo: Stock = stocks[stocks.len() - 1].clone();
    let dataset: Vec<Stock> = stocks[0..stocks.len() - 1].to_vec();

    let (training_set, test_set) = split_data(&dataset, 0.8);

    let table_builder: TableBuilder = construct_table(&training_set);

    let table: Table = table_builder.build().unwrap();

    let classifier: RandomForestClassifier = RandomForestClassifierOptions::new().fit(Gini, table);

    let num_tests: f32 = test_set.len() as f32;
    let mut num_correct: f32 = 0.0;

    for stock in test_set {
        let result = classifier.predict(&stock.get_array());

        if result == stock.get_label() {
            num_correct += 1.0;
        }
    }

    let mut accuracy = num_correct / num_tests;
    let mut switch_flag: bool = false;

    if accuracy < 0.5 {
        accuracy = 1.0 - accuracy;
        switch_flag = true;
    }

    let mut result = classifier.predict(&ultimo.get_array());

    if switch_flag {
        result = if result == 1.0 { 0.0 } else { 1.0 };
    }

    (result, accuracy)
}
