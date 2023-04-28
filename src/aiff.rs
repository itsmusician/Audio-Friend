// Audio Interchange File Format (Audio IFF, AIFF, .aif, .aiff)

use std::env;
use std::fs::File;
use std::io::prelude::*;
use extended::Extended;

fn write() -> std::io::Result<()> {
    // Pass in Values: channel count, bit depth, sample rate, and data size
    let args: Vec<String> = env::args().collect();
    let name = format!("{}", &args[1]) + ".aif";
    let channels: u16 = args[2].parse().unwrap();
    let bit_depth: u16 = args[3].parse().unwrap();
	let sample_rate: Extended = Extended::from(args[4].parse::<f64>().unwrap());
    let data_size: u32 = args[5].parse().unwrap();

	// Create output file
    let mut file = File::create("output/".to_owned() + &name)?;
    
    // Write the header :)
    // FORM AIFF chunk
    file.write_all(b"FORM")?;
    file.write_all(&(data_size + 46).to_be_bytes())?;
    file.write_all(b"AIFF")?;

    // Common local chunk
    file.write_all(b"COMM")?;
    file.write_all(&18u32.to_be_bytes())?;
    file.write_all(&channels.to_be_bytes())?;
	file.write_all(&(data_size/(channels as u32)).to_be_bytes())?;
	file.write_all(&bit_depth.to_be_bytes())?;
    file.write_all(&sample_rate.to_be_bytes())?;

	// Sound Data local chunk
	file.write_all(b"SSND")?;
	file.write_all(&(data_size + 8).to_be_bytes())?;
	file.write_all(&0u32.to_be_bytes())?;
	file.write_all(&0u32.to_be_bytes())?;

    // Write data :D
    
    Ok(())
}