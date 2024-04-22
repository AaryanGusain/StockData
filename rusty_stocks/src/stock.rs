#[derive(Debug)]
pub struct Stock {
    pub date: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub adj_close: f32,
    pub volume: usize,
}

impl Stock {
    fn get_array(&self) -> [f32; 6] {
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
