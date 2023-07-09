use cpal::{BuildStreamError, Stream};
use cpal::traits::DeviceTrait;

pub use ltc_frame::LtcFrame;
pub use reader::LtcFrameReader;

pub mod filter;
mod reader;
mod ltc_frame;


pub fn connect_stream<Callback, CallbackError>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    callback: Callback,
    error_callback: CallbackError,
) -> Result<Stream, BuildStreamError>
    where Callback: Fn((LtcFrame, u128)) + Send + 'static,
          CallbackError: Fn(cpal::StreamError) + Send + 'static,
{
    let mut filter_lowpass = filter::RawLowpassFilter::new();
    let mut filter_denoise = filter::RawDenoiseFilter::new();
    let mut filter_demod = filter::RawDemodFilter::new();
    let mut filter_fm_denoise = filter::FmDenoiseFilter::new();
    let mut filter_fm_decode = filter::FmDecodeFilter::new();
    let mut frame_reader = LtcFrameReader::new();

    device.build_input_stream(
        config,
        move |data: &[f32], _info: &cpal::InputCallbackInfo| {
            let data = data.to_vec();
            let data = filter_lowpass.filter(&data);
            let data = filter_denoise.filter(&data);
            let data = filter_demod.filter(&data);
            let data = filter_fm_denoise.filter(&data);
            let data = filter_fm_decode.filter(&data);
            let data = frame_reader.read(&data);
            let fps_actual = frame_reader.frame_rate()
                .unwrap_or(0);
            for frame in data {
                callback((frame, fps_actual));
            }
        },
        move |err| {
            error_callback(err)
        },
        None,
    )
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs::File;

    use crate::filter::{FmDecodeFilter, FmDenoiseFilter, RawDemodFilter, RawDenoiseFilter, RawLowpassFilter};
    use crate::LtcFrameReader;

    type Err = Box<dyn Error>;

    #[test]
    fn foobar() -> Result<(), Err> {
        let file = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test.json");
        let file = File::open(file)?;
        let file: Vec<f32> = serde_json::from_reader(file)?;
        assert_eq!(&file.as_slice()[0..100], &[
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.46377563,
            0.46377563, -1.0, -1.0, -0.59332275, -1.0, -0.57785034, -1.0, -0.5496216, -1.0,
            -0.79473877, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -0.9883728, -1.0,
            -0.9460449, -1.0, -0.89746094, -1.0, -0.82821655, -1.0, -1.0, -1.0, -1.0, -1.0,
            -0.9974365, -1.0, -0.99923706, -1.0, -1.0, -1.0, -1.0, -1.0, -0.88964844, -1.0,
            0.9953613, -0.71066284, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.59994507, 0.59994507, -1.0, -1.0,
            -0.5836487, -1.0
        ]);
        let mut filter_lowpass = RawLowpassFilter::new();
        let file = filter_lowpass.filter(&file);
        assert_eq!(&file.as_slice()[0..100], &[
            0.1999939, 0.3999878, 0.59998167, 0.7999756, 0.9999695, 0.9999695, 0.8927307,
            0.78549194, 0.38549805, -0.01449585, -0.3331543, -0.62590945, -0.8342346, -0.8342346,
            -0.7441589, -0.8254944, -0.7844421, -0.86887205, -0.86887205, -0.9589478, -0.9589478,
            -1.0, -1.0, -1.0, -1.0, -1.0, -0.9976746, -0.9976746, -0.9868835, -0.9868835,
            -0.9663757, -0.9687012, -0.9343445, -0.9451355, -0.9451355, -0.9656433, -0.9656433,
            -1.0, -0.9994873, -0.9994873, -0.9993347, -0.9993347, -0.9993347, -0.9998474,
            -0.9998474, -1.0, -0.9779297, -0.9779297, -0.5788574, -0.52099, -0.120996095,
            0.2569275, 0.6569214, 0.657843, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695, 0.9999695,
            0.9999695, 0.9199646, 0.83995974, 0.4399658, 0.039971925, -0.2767517, -0.5967407
        ]);
        let mut filter_denoise = RawDenoiseFilter::new();
        let file = filter_denoise.filter(&file);
        assert_eq!(&file.as_slice()[0..100], &[
            1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1
        ]);
        let mut filter_demod = RawDemodFilter::new();
        let file = filter_demod.filter(&file);
        assert_eq!(&file.as_slice()[0..100], &[
            0, 8, 41, 46, 40, 46, 18, 24, 40, 46, 19, 23, 41, 45, 41, 45, 41, 23, 19, 45, 41, 45,
            41, 46, 40, 24, 18, 46, 40, 46, 40, 46, 41, 45, 19, 23, 41, 45, 19, 23, 41, 46, 40, 46,
            19, 23, 40, 46, 40, 46, 40, 46, 40, 46, 41, 23, 19, 23, 19, 23, 19, 23, 19, 24, 18, 24,
            18, 24, 18, 24, 18, 24, 18, 24, 18, 24, 18, 24, 19, 45, 19, 23, 19, 23, 19, 23, 19, 23,
            41, 46, 40, 46, 40, 46, 40, 46, 40, 46, 40, 46
        ]);
        let mut filter_fm_denoise = FmDenoiseFilter::new();
        let file = filter_fm_denoise.filter(&file);
        assert_eq!(&file.as_slice()[0..100], &[
            49, 46, 40, 46, 18, 24, 40, 46, 19, 23, 41, 45, 41, 45, 41, 23, 19, 45, 41, 45, 41, 46,
            40, 24, 18, 46, 40, 46, 40, 46, 41, 45, 19, 23, 41, 45, 19, 23, 41, 46, 40, 46, 19, 23,
            40, 46, 40, 46, 40, 46, 40, 46, 41, 23, 19, 23, 19, 23, 19, 23, 19, 24, 18, 24, 18, 24,
            18, 24, 18, 24, 18, 24, 18, 24, 18, 24, 19, 45, 19, 23, 19, 23, 19, 23, 19, 23, 41, 46,
            40, 46, 40, 46, 40, 46, 40, 46, 40, 46, 40, 46
        ]);
        let mut filter_fm_decode = FmDecodeFilter::new();
        let file = filter_fm_decode.filter(&file);
        assert_eq!(&file.as_slice()[0..100], &[
            false, false, false, false, true, false, false, true, false, false, false, false, false,
            true, false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, true, false, false, true, false, false, false, false, true, false, false,
            false, false, false, false, false, false, false, true, true, true, true, true, true,
            true, true, true, true, true, true, false, true, true, true, true, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, true, false, false, false, false, false, false,
        ]);
        let mut frame_reader = LtcFrameReader::new();
        let file = frame_reader.read(&file);
        assert_eq!(file[0].hour(), 19);
        assert_eq!(file[0].minute(), 12);
        assert_eq!(file[0].second(), 10);
        assert_eq!(file[0].frame(), 7);
        for frame in file {
            println!("{:?}", frame);
        }

        Ok(())
    }
}