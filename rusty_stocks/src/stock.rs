use core::fmt;

#[derive(Debug, Clone)]
pub enum Tomorrow {
    Increase,
    Decrease,
    Predict,
}

#[derive(Debug, Clone)]
pub struct Stock {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: usize,
    tomorrow: Tomorrow,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        date: String,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        adj_close: f64,
        volume: usize,
        tomorrow: Tomorrow,
    ) -> Self {
        Self {
            date,
            open,
            high,
            low,
            close,
            adj_close,
            volume,
            tomorrow,
        }
    }

    /*
        Returns array of stock data used as the features of the random forest

        @param (&self) current stock object

        @return ([f64; 6]) array of length six holding the open, high, low, adj_close, close and volume data
    */
    pub fn get_array(&self) -> [f64; 6] {
        [
            self.open,
            self.high,
            self.low,
            self.adj_close,
            self.close,
            self.volume as f64,
        ]
    }

    pub fn get_label(&self) -> f64 {
        match self.tomorrow {
            Tomorrow::Increase => 1.0,
            Tomorrow::Decrease => 0.0,
            Tomorrow::Predict => -1.0,
        }
    }

    pub fn get_open(&self) -> f64 {
        self.open
    }

    pub fn get_close(&self) -> f64 {
        self.close
    }

    pub fn set_tomorrow(&mut self, tomorrow: Tomorrow) {
        self.tomorrow = tomorrow;
    }
}
