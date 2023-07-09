pub struct FmDenoiseFilter {
    last: i64,
}

impl FmDenoiseFilter {
    pub fn new() -> FmDenoiseFilter {
        return FmDenoiseFilter {
            last: 0,
        };
    }

    pub fn filter(&mut self, data: &Vec<i64>) -> Vec<i64> {
        let bit_length = *data.iter().max().unwrap_or(&0);
        data.into_iter().filter_map(|&val| {
            self.last += val;
            if self.last > bit_length / 4 {
                let value = self.last;
                self.last = 0;
                Some(value)
            } else {
                None
            }
        }).collect()
    }
}