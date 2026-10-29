mod core;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::{capture::capture, clipboard::copy};
use clap::Parser;

/// Yet another screenshot tool.

#[derive(Parser, Debug)]
#[command(version, about, version)]
struct Args {
    /// If you want save the ss as a file
    #[arg(short, long, value_name = "PATH")]
    save: Option<String>,
}

fn main() {
    let args = Args::parse();
    let image = match capture() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    match args.save {
        Some(path) => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();
            let path = format!("{path}/simpless-{now}.png");
            if let Err(e) = image.save(path) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        _ => {
            if let Err(e) = copy(image) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}
