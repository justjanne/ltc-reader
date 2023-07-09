pub struct RawDenoiseFilter {}

impl RawDenoiseFilter {
    pub fn new() -> RawDenoiseFilter {
        return RawDenoiseFilter {};
    }

    pub fn filter(&mut self, data: &Vec<f32>) -> Vec<i8> {
        return data.iter().map(|&val| {
            if val > 0.0 { 1 } else { -1 }
        }).collect();
    }
}