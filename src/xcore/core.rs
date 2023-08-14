/******************************
 *  Copyright (c) xTekC.      *
 *  Licensed under MPL-2.0.   *
 *  See LICENSE for details.  *
 *                            *
 ******************************/

use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::metadata;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
const CD_DEVICE_PATH: &str = "/dev/sr0";
// Add other platform-specific paths

const BLOCK_SIZE: usize = 4 * 1024 * 1024; // 4M

pub fn core_main(output_name: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let output_path = construct_output_path(output_name, output_dir)?;

    let mut input = File::open(CD_DEVICE_PATH)?;
    println!("Device found: {}", CD_DEVICE_PATH);

    let total_size = metadata(CD_DEVICE_PATH)?.len();

    let pb = ProgressBar::new(total_size);
    println!("Ripping {}.iso to {}", output_name, output_dir);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.magenta/purple}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));

    let mut output = create_output_file(&output_path)?;

    let mut buffer = vec![0u8; BLOCK_SIZE];
    let mut bytes_read_total: u64 = 0;

    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        bytes_read_total += bytes_read as u64;
        pb.set_position(bytes_read_total);

        output.write_all(&buffer[..bytes_read])?;
    }

    pb.finish_with_message("completed");

    Ok(())
}

fn construct_output_name(name: &str) -> Result<String, Box<dyn Error>> {
    if name.ends_with(".iso") {
        Ok(name.to_string())
    } else {
        Ok(format!("{}.iso", name))
    }
}

fn construct_output_path(name: &str, dir: &str) -> Result<PathBuf, Box<dyn Error>> {
    let output_name = construct_output_name(name)?;
    Ok(Path::new(dir).join(output_name))
}

fn create_output_file(path: &Path) -> Result<File, Box<dyn Error>> {
    let output = File::create(path)?;
    Ok(output)
}
