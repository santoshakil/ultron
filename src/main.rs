use clap::Parser;
use ultron::args::Args;

pub fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
