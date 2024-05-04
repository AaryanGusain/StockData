## Cloning

git clone https://github.com/DerekW3/CS128_project.git

## Data

Historical data can be downloaded from YahooFinance or feel free to use the included data under
CS128_project/tests/inputs

## File Path

Inputted file path is relative to the home directory rusty_stocks

## Running

cd into rust_stocks directory, cargo run can be run with more than one file if desired

cargo run -- <file path>

### Examples:

cargo run -- tests/inputs/Amazon.csv

cargo run -- tests/inputs/Apple\ Stock\ Historical.csv tests/inputs/PepsiCo\ Stock\ Data.csv

### Beware:

Random forest fitting is a slow operation and passing a file with large amounts of data (for example one year of
historical data)
will be slow, but it is not crashing, just give it a minute!

## Plotting
Outputs a file in the project home directory with the plot

cargo run plot <file path>

### Examples:

cargo run plot tests/inputs/Amazon.csv
