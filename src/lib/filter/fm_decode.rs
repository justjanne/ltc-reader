pub struct FmDecodeFilter {
    remaining: bool,
}

impl FmDecodeFilter {
    pub fn new() -> FmDecodeFilter {
        return FmDecodeFilter {
            remaining: false,
        };
    }

    pub fn filter(&mut self, data: &Vec<i64>) -> Vec<bool> {
        let min = *data.iter().min().unwrap_or(&0);
        let max = *data.iter().max().unwrap_or(&0);
        let threshold = (max - min) / 2;
        data.into_iter().filter_map(|&val| {
            let val = val - min;
            let is_zero = val > threshold;
            if !is_zero {
                if self.remaining {
                    self.remaining = false;
                    Some(true)
                } else {
                    self.remaining = true;
                    None
                }
            } else {
                if self.remaining {
                    self.remaining = false;
                    None
                } else {
                    Some(false)
                }
            }
        }).collect()
    }
}
