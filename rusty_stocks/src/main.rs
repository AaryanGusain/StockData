mod plot;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "plot" {
        // Plot the <data.csv> if the command input is "cargo run plot <path to data.csv>"
        let file_path = &args[2];
        if let Err(err) = plot::plot_csv(file_path) {
            eprintln!("Error: {}", err);
        }
    } else if let Err(e) = rusty_stocks::get_args().and_then(rusty_stocks::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
