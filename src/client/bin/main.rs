use ureq;
use clap::{Arg, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
    content: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello World!");
}
