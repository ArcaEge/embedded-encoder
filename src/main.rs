use clap::{Parser, Subcommand};
use clio::*;

/// An asset/level encoder for embedded-engine to encode/decode assets and levels into and from binary format
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Encode/decode an asset
    Asset {
        /// Input file, use "-" for stdin
        #[clap(value_parser)]
        input: Input,

        /// Output file
        #[clap(value_parser)]
        output: Output,
    },
    /// Encode/decode a level
    Level {
        /// Input file, use "-" for stdin
        #[clap(value_parser)]
        input: Input,

        /// Output file
        #[clap(value_parser)]
        output: Output,
    },
}

fn main() {
    let args = Cli::parse();
}
