pub struct RawLowpassFilter {
    last: [f32; 4],
}

impl RawLowpassFilter {
    pub fn new() -> RawLowpassFilter {
        return RawLowpassFilter {
            last: [0.0, 0.0, 0.0, 0.0],
        };
    }

    pub fn filter(&mut self, data: &Vec<f32>) -> Vec<f32> {
        return data.iter().map(|&val| {
            let next = (self.last[0] + self.last[1] + self.last[2] + self.last[3] + val) / 5.0;
            self.last = [self.last[1], self.last[2], self.last[3], val];
            next
        }).collect();
    }
}