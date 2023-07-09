use std::fmt::Formatter;

#[derive(Eq, PartialEq, Clone)]
pub struct LtcFrame {
    frame: u8,
    second: u8,
    minute: u8,
    hour: u8,
    flag_drop: bool,
    flag_color: bool,
    flag_clock: bool,
    userdata_format: Vec<bool>,
    userdata: Vec<bool>,
}


fn read_digit(data: &[bool]) -> u8 {
    let mut result: u8 = 0;
    if data.len() >= 1 {
        result += data[0] as u8;
    }
    if data.len() >= 2 {
        result += (data[1] as u8) << 1;
    }
    if data.len() >= 3 {
        result += (data[2] as u8) << 2;
    }
    if data.len() >= 4 {
        result += (data[3] as u8) << 3;
    }
    return result;
}

fn read_bcd(one: &[bool], ten: &[bool]) -> u8 {
    return read_digit(one) + read_digit(ten) * 10;
}


impl LtcFrame {
    pub fn read(data: &[bool]) -> LtcFrame {
        let frame = read_bcd(&data[0..4], &data[8..10]);
        let second = read_bcd(&data[16..20], &data[24..27]);
        let minute = read_bcd(&data[32..36], &data[40..43]);
        let hour = read_bcd(&data[48..52], &data[56..58]);
        let flag_drop = data[10];
        let flag_color = data[11];
        let flag_clock = data[58];
        let userdata_format = vec![data[27], data[43], data[59]];
        let mut userdata: Vec<bool> = vec![];
        userdata.extend_from_slice(&data[4..8]);
        userdata.extend_from_slice(&data[12..16]);
        userdata.extend_from_slice(&data[20..24]);
        userdata.extend_from_slice(&data[28..32]);
        userdata.extend_from_slice(&data[36..40]);
        userdata.extend_from_slice(&data[44..48]);
        userdata.extend_from_slice(&data[52..56]);
        userdata.extend_from_slice(&data[60..64]);
        return LtcFrame {
            frame,
            second,
            minute,
            hour,
            flag_drop,
            flag_color,
            flag_clock,
            userdata_format,
            userdata,
        };
    }

    pub fn frame(&self) -> u8 {
        self.frame
    }
    pub fn second(&self) -> u8 {
        self.second
    }
    pub fn minute(&self) -> u8 {
        self.minute
    }
    pub fn hour(&self) -> u8 {
        self.hour
    }
    pub fn is_drop(&self) -> bool {
        self.flag_drop
    }
    pub fn is_color(&self) -> bool {
        self.flag_color
    }
    pub fn is_clock(&self) -> bool {
        self.flag_clock
    }
    pub fn userdata_format(&self) -> &[bool] {
        self.userdata_format.as_slice()
    }
    pub fn userdata(&self) -> &[bool] {
        self.userdata.as_slice()
    }
}

impl std::fmt::Display for LtcFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}:{:03}",
               self.hour(), self.minute(), self.second(), self.frame(),
        )
    }
}

impl std::fmt::Debug for LtcFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut flags = vec![];
        if self.is_color() {
            flags.push("color");
        }
        if self.is_drop() {
            flags.push("drop");
        }
        if self.is_clock() {
            flags.push("clock");
        }
        let flags = flags.join(",");
        if f.alternate() {
            write!(f, "LtcFrame {{\n  timecode={}\n  flags=[{}]\n  userdata_format={}\n  userdata={}\n}}",
                   self.to_string(),
                   flags,
                   format_bits(self.userdata_format()),
                   format_bits(self.userdata())
            )
        } else {
            write!(f, "LtcFrame {{ timecode={}, flags=[{}], userdata_format={}, userdata={} }}",
                   self.to_string(),
                   flags,
                   format_bits(self.userdata_format()),
                   format_bits(self.userdata())
            )
        }
    }
}

fn format_bits(data: &[bool]) -> String {
    return data.iter()
        .map(|&bit| if bit { "1" } else { "0" })
        .collect::<Vec<_>>().join("");
}
