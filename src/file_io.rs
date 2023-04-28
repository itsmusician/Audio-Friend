
/*
    // Define iterator & max sample value
    const MAXSAMPLE: i16 = i16::MAX;

    // Write Data
    for i in data_size/2 {
        file.write_all(&(testsample).to_be_bytes())?;
    }
*/

use std::env;

#[derive(Debug)]
struct AudioInfo {
    name: String,
    extension: String,
    channels: u16,
    bit_depth: f32,
    sample_rate: f64,
    data_size: u32,
}

impl AudioInfo {
    fn cli(args: &Vec<String>) -> Self {
        Self {
            name: format!("{}", &args[1]),
            extension: format!("{}", &args[2]),
            channels: args[3].parse().unwrap(),
            bit_depth: args[4].parse().unwrap(),
            sample_rate: args[5].parse().unwrap(),
            data_size: args[6].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum FileFormat {
    RAW(AudioInfo),
    WAFF(AudioInfo),
    AIFF(AudioInfo),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let output = FileFormat::WAFF(AudioInfo::cli(&args));
    let output = AudioInfo::cli(&args);

    println!("{:?}", output);
}