use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

pub fn plot_csv(path: &str) -> Result<(), Box<dyn Error>> {
    // Read the CSV file
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Read the headers
    let headers = rdr.headers()?.clone();
    let header_names: Vec<String> = headers.iter().map(|h| h.to_string()).collect();

    // Initialize plotting area
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Stock Prices", ("sans-serif", 40).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..250f64, 100f64..300f64)?; // Dimensions of the plot. 

    chart.configure_mesh().draw()?;

    // Plot each column except "Date" and "Volume"
    for (i, header) in header_names.iter().enumerate() {
        if header != "Date" && header != "Volume" {
            let mut data = Vec::new();
            for result in rdr.records() {
                let record = result?;
                let value: f64 = record[i].parse()?;
                data.push(value);
            }

            chart
                .draw_series(LineSeries::new(
                    (0..data.len()).map(|x| (x as f64, data[x])),
                    &Palette99::pick(i),
                ))?
                .label(header)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &Palette99::pick(i)));

            chart
                .configure_series_labels()
                .border_style(&BLACK)
                .background_style(&WHITE.mix(0.8))
                .position(SeriesLabelPosition::UpperLeft)
                .draw()?;
            
            rdr = ReaderBuilder::new().has_headers(true).from_reader(File::open(path)?);
        }
    }

    Ok(())
}