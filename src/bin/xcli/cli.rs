/******************************
 *  Copyright (c) xTekC.      *
 *  Licensed under MPL-2.0.   *
 *  See LICENSE for details.  *
 *                            *
 ******************************/

use clap::Parser;
use ripi::xcore::core::core_main;

/// Rip an ISO from media
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the ISO
    #[arg(short = 'n', long = "name")]
    output_name: String,

    /// Output directory of the ISO
    #[arg(short = 'o', long = "output")]
    output_dir: String,
}

pub async fn cli_main() {
    let args = Args::parse();

    match core_main(&args.output_name, &args.output_dir) {
        Ok(_) => println!("ISO rip successful."),
        Err(e) => eprintln!("Error during ISO rip: {}", e),
    }
}
