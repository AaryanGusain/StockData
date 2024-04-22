use core::fmt;

#[derive(Debug)]
pub struct Stock {
    date: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    adj_close: f32,
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
        open: f32,
        high: f32,
        low: f32,
        close: f32,
        adj_close: f32,
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

    pub fn get_array(&self) -> [f32; 6] {
        [
            self.open,
            self.high,
            self.low,
            self.close,
            self.adj_close,
            self.volume as f32,
        ]
    }
}
