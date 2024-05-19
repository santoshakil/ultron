use clap::Parser;

use crate::file::file::Io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub io: Io,
}
