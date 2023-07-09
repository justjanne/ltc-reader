use crate::ltc_frame::LtcFrame;

const LTC_FRAME_HEADER: &[bool] = &[
    false, false, true, true,
    true, true, true, true,
    true, true, true, true,
    true, true, false, true
];

pub struct LtcFrameReader {
    buffer: Vec<bool>,
    last_frame_count: u128,
    frame_rate: Option<u128>,
}

impl LtcFrameReader {
    pub fn new() -> LtcFrameReader {
        return LtcFrameReader {
            buffer: vec![],
            last_frame_count: 0,
            frame_rate: None,
        };
    }

    pub fn frame_rate(&self) -> Option<u128> {
        return self.frame_rate;
    }

    pub fn read(&mut self, data: &Vec<bool>) -> Vec<LtcFrame> {
        return data.into_iter().filter_map(|&bit| {
            let mut result: Option<LtcFrame> = None;
            if self.buffer.ends_with(LTC_FRAME_HEADER) {
                if self.buffer.len() == 80 {
                    let frame = LtcFrame::read(self.buffer.as_slice());
                    let frame_id = frame.frame() as u128;
                    if frame_id < self.last_frame_count {
                        self.frame_rate = Some(self.last_frame_count + 1);
                        self.last_frame_count = frame_id;
                    }
                    self.last_frame_count = frame_id;
                    result = Some(frame);
                } else {
                    self.last_frame_count = 0;
                    self.frame_rate = None;
                }
                self.buffer.clear();
            }
            self.buffer.push(bit);
            result
        }).collect();
    }
}