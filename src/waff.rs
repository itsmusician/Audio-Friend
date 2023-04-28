// Waveform Audio File Format (WAFF, .wav, .wave)

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn write() -> std::io::Result<()> {
    // Pass in Values: channel count, bit depth, sample rate, and data size
    let args: Vec<String> = env::args().collect();
    let name = format!("{}", &args[1]) + ".wav";
    let channels: u16 = args[2].parse().unwrap();
    let bit_depth: u16 = args[3].parse().unwrap();
	let sample_rate: u32 = args[4].parse().unwrap();
    let data_size: u32 = args[5].parse().unwrap();

	// Calculate Values: byte rate, block align
    let byte_rate: u32 = sample_rate * bit_depth as u32 * channels as u32 / 8;
    let block_align: u16 = ((bit_depth as u32 * channels as u32) / 8) as u16;
    
	// Create output file
    let mut file = File::create("output/".to_owned() + &name)?;
    
    // Write the header :)
    // RIFF chunk
    file.write_all(b"RIFF")?;
    file.write_all(&(data_size + 36).to_le_bytes())?;
    file.write_all(b"WAVE")?;

    // fmt subchunk
    file.write_all(b"fmt ")?;
    file.write_all(&16u32.to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&channels.to_le_bytes())?;
    file.write_all(&sample_rate.to_le_bytes())?;
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&block_align.to_le_bytes())?;
    file.write_all(&bit_depth.to_le_bytes())?;

    // data subchunk
    file.write_all(b"data")?;
    file.write_all(&data_size.to_le_bytes())?;

    // Write data :D
    
    Ok(())
}