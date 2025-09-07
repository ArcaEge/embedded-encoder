use super::{Cli, types::Sound, types::SoundTone};
use clap::{CommandFactory, Error, error::ErrorKind};
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind, num::u7};
use postcard::to_allocvec;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Encodes the sound
pub fn encode_sound(track: u8, input: PathBuf, output: PathBuf) -> Result<(), Error> {
    let midi_bytes = fs::read(&input).map_err(|_| {
        Cli::command().error(
            ErrorKind::Io,
            format!("failed to open input file {:#?}", &input),
        )
    })?;

    let smf = Smf::parse(&midi_bytes).map_err(|_| {
        Cli::command().error(
            ErrorKind::InvalidValue,
            format!(
                "failed to parse input file {:#?}, file seems to be invalid or corrupted",
                &input
            ),
        )
    })?;

    let track0 = smf.tracks.get(0).ok_or_else(|| {
        Cli::command().error(
            ErrorKind::InvalidValue,
            format!("midi file doesn't have any tracks"),
        )
    })?;

    let track1 = smf.tracks.get(track as usize).ok_or_else(|| {
        Cli::command().error(
            ErrorKind::InvalidValue,
            format!(
                "midi file doesn't have track {track}, use --track 0 if this is a type-0 MIDI file"
            ),
        )
    })?;

    // Tempo measured in us per quarter note
    let tempo: u64 = track0
        .iter()
        .find_map(|event| {
            if let TrackEventKind::Meta(MetaMessage::Tempo(tempo)) = event.kind {
                Some(tempo.as_int().into())
            } else {
                None
            }
        })
        .unwrap_or(500_000); // 500_000 us per quarter note = 120 bpm

    let ticks_per_quarter: u64 = if let Timing::Metrical(ticks) = smf.header.timing {
        Some(ticks.as_int().into())
    } else {
        None
    }
    .ok_or_else(|| {
        Cli::command().error(
            ErrorKind::InvalidValue,
            format!("midi file uses timecode-based timing, only metrical timing is supported"),
        )
    })?;

    let us_per_tick: u64 = tempo / ticks_per_quarter;

    let mut sound = Sound::new();

    // Frequency, start time. Only one note is assumed to be active at a time
    let mut active_note: (f32, u64) = (0.0, 0);

    // Running total of event.delta (i.e. current midi time in ticks), uses u64 because why not
    let mut current_time: u64 = 0;

    for event in track1.iter() {
        current_time += event.delta.as_int() as u64;

        if let TrackEventKind::Midi {
            channel: _,
            message,
        } = event.kind
        {
            if let MidiMessage::NoteOn { key, vel: _ } = message {
                let time_difference_ticks = current_time - active_note.1;

                if time_difference_ticks > 0 {
                    sound.push(SoundTone {
                        freq: active_note.0,
                        length_us: time_difference_ticks * us_per_tick,
                    });
                    println!(
                        "Tone: {:?} {:?}",
                        active_note.0,
                        time_difference_ticks * us_per_tick
                    );
                }

                active_note = (midi_to_freq(key), current_time);
            } else if let MidiMessage::NoteOff { key, vel: _ } = message {
                if active_note.0 == midi_to_freq(key) {
                    let time_difference_ticks = current_time - active_note.1;

                    if time_difference_ticks > 0 {
                        sound.push(SoundTone {
                            freq: active_note.0,
                            length_us: time_difference_ticks * us_per_tick,
                        });
                        println!(
                            "Tone: {:?} {:?}",
                            active_note.0,
                            time_difference_ticks * us_per_tick
                        );
                    }

                    active_note = (0.0, current_time);
                }
            }
        }
    }

    let bytes: Vec<u8> = to_allocvec(&sound).map_err(|_| {
        Cli::command().error(ErrorKind::InvalidValue, "failed to serialize sprite data")
    })?;

    let mut out_file = fs::File::create(&output).map_err(|_| {
        Cli::command().error(
            ErrorKind::Io,
            format!("failed to open output file {:#?}", &output),
        )
    })?;

    out_file.write_all(&bytes).map_err(|_| {
        Cli::command().error(
            ErrorKind::Io,
            format!("failed to write to output file {:#?}", &output),
        )
    })?;

    println!();
    println!("Successfully wrote output to file: {:#?}", &output);

    Ok(())
}

fn midi_to_freq(midi_tone: u7) -> f32 {
    440.0 * (2f32.powf((midi_tone.as_int() as i32 - 69) as f32 / 12f32))
}
