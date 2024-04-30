use core::fmt;

#[derive(Debug, Clone)]
pub struct Stock {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: usize,
}

impl fmt::Display for Stock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: Open - {}, High - {}, Low - {}, Close - {}, Volume - {}",
            self.date, self.open, self.high, self.low, self.close, self.volume
        )
    }
}

impl Stock {
    pub fn new(
        date: String,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        adj_close: f64,
        volume: usize,
    ) -> Self {
        Self {
            date,
            open,
            high,
            low,
            close,
            adj_close,
            volume,
        }
    }

    pub fn get_array(&self) -> [f64; 5] {
        [
            self.open,
            self.high,
            self.low,
            self.adj_close,
            self.volume as f64,
        ]
    }

    pub fn get_label(&self) -> f64 {
        if self.close > self.open {
            1.0
        } else {
            0.0
        }
    }
}
