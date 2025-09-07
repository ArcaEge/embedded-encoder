mod levels;
mod midi;
mod spritesheet;
mod types;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// An asset encoder for embedded-engine to encode spritesheets, levels and midi files into and from binary format
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Encode a spritesheet into embsprite format.
    /// Input must be in a vertical strip arrangement, 1 sprite wide
    Spritesheet {
        /// Height of each sprite in pixels. Width is auto-determined as the width of the image
        #[arg(short = 'H', long, default_value_t = 8, value_parser = spritesheet::validate_height)]
        height: u32,

        /// Input file
        #[clap(value_parser)]
        input: PathBuf,

        /// Output file
        #[clap(value_parser)]
        output: PathBuf,
    },
    /// Encode a level - this is currently a work in progress and doesn't work
    Level {
        /// Input file
        #[clap(value_parser)]
        input: PathBuf,

        /// Output file
        #[clap(value_parser)]
        output: PathBuf,
    },
    /// Encode a MIDI file
    Midi {
        /// The index of the instrument track to use
        #[arg(short, long, default_value_t = 1)]
        track: u8,

        /// Input file
        #[clap(value_parser)]
        input: PathBuf,

        /// Output file
        #[clap(value_parser)]
        output: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Spritesheet {
            height,
            input,
            output,
        } => spritesheet::encode_spritesheet(height, input, output).unwrap_or_else(|e| e.exit()),

        Commands::Level { input, output } => todo!(),

        Commands::Midi {
            track,
            input,
            output,
        } => midi::encode_sound(track, input, output).unwrap_or_else(|e| e.exit()),
    }
}
