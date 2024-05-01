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
    log_return: f64,
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
            log_return: 0.0,
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

    /*
        Returns the label (increasing or decreasing) of the current stock object

        @param (&self) current stock object

        @return (f64) 1 for increasing, 0 for decreasing, -1 in the case this should not be read
    */
    pub fn get_label(&self) -> f64 {
        match self.tomorrow {
            Tomorrow::Increase => 1.0,
            Tomorrow::Decrease => 0.0,
            Tomorrow::Predict => -1.0,
        }
    }

    /*
        Simple getter for open attribute used in determining the label of an already determined stock

        @param (&self) current stock object

        @return (f64) open attribute
    */
    pub fn get_open(&self) -> f64 {
        self.open
    }

    /*
        Simple getter for close attribute used in determining the label of an already determined stock

        @param (&self) current stock object

        @return (f64) close attribute
    */
    pub fn get_close(&self) -> f64 {
        self.close
    }

    /*
        Simple getter to get average price of the stock

        @param (&self) current stock object

        @return (f64) average stock price for current stock
    */
    pub fn get_price(&self) -> f64 {
        (self.high + self.close) / 2.0
    }

    pub fn get_return(&self) -> f64 {
        self.log_return
    }

    /*
        Simple setter for tomorrow attribute used in determining the label of an already determined stock

        @param (&mut self) current stock object
    */
    pub fn set_tomorrow(&mut self, tomorrow: Tomorrow) {
        self.tomorrow = tomorrow;
    }

    /*
        Setter function for the logarithmic return on the current stock

        @param (&mut self) current stock object
        @param (price_yesterday: f64) price of stock from the day before
    */
    pub fn set_return(&mut self, price_yesterday: f64) {
        self.log_return = (self.get_price() / price_yesterday).ln();
    }
}
