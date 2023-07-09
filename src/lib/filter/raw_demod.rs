pub struct RawDemodFilter {
    last: i8,
    count: i64,
}

impl RawDemodFilter {
    pub fn new() -> RawDemodFilter {
        return RawDemodFilter {
            last: 0,
            count: 0,
        };
    }

    pub fn filter(&mut self, data: &Vec<i8>) -> Vec<i64> {
        data.into_iter().filter_map(|&val| {
            if val == self.last {
                self.count += 1;
                None
            } else {
                let count = self.count;
                self.count = 0;
                self.last = val;
                Some(count)
            }
        }).collect()
    }
}