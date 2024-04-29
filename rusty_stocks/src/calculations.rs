use crate::stock::{self, Stock};

use clap::Error;
use randomforest::criterion::Mse;
use randomforest::table::{self, Table, TableBuilder, TableError};
use randomforest::RandomForestRegressorOptions;

pub fn construct_table(stocks: &Vec<Stock>) -> TableBuilder {
    let mut table_builder: TableBuilder = TableBuilder::new();

    for stock in stocks {
        let _ = table_builder.add_row(&stock.get_array(), stock.get_label());
    }

    table_builder
}
