// RAW Audio (PCM, .raw, .pcm, .sam, extensionless)

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn write() -> std::io::Result<()> {
	// Create output file
    let name = format!("{}", &args[1]) + ".raw";
    let mut file = File::create("output/".to_owned() + &name)?;

    // Write data :D
    
    Ok(())
}