mod core;
use clap::Parser;
use core::capture;

/// Yet another screenshot tool.

#[derive(Parser, Debug)]
#[command(version, about, version = "1.0.0")]
struct Args {
    /// If you want save the ss as a file
    #[arg(short, long, value_name = "PATH")]
    save: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.save {
        Some(save) => {
            println!("save");
        }
        _ => {
            println!("clipboard")
        }
    }
}
