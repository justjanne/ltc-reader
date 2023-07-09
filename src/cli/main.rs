use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use ltc_reader::connect_stream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device available");
    let default_config = device.default_input_config().expect("no supported config");
    let stream = connect_stream(
        &device,
        &default_config.config(),
        |(frame, fps)| {
            println!("{:} fps {:}", fps, frame);
        },
        |err| {
            println!("error in stream: {:}", err);
        },
    )?;
    stream.play().expect("could not start stream");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
