# Embedded Encoder

An asset encoder for my game engine, [embedded-engine](https://github.com/ArcaEge/embedded-engine), used for encoding assets into binary form.

Supports encoding:

- Spritesheets
- Levels - WIP
- Music (from type-1 MIDI files with a single instrument channel)

## Help

Run with a `--help` flag to show a list of available commands. Then use `command_name --help` to view the help page of the command.

### Levels - currently a WIP

Levels are designed in LDtk. Use a Tiles-style layer called "Main" for the main layer, "Foreground" for tiles in the foreground (i.e. those that should be displayed in front of the player) and "Background" for background tiles. Assign custom data to each tile (click on the tile on the tileset screen) with its index in the encoded spritesheet as a plain integer. Interactive items like doors, chests, coins and NPCs should be on an Entity-style layer called "Entities".

### MIDI

Currently only Type-1 MIDI files (which is the most common format anyway) are supported and only tracks 0 (tempo track) and 1 (first instrument track) are parsed. Also, tempo changes are not supported and anything but the first tempo will be ignored. Pitch bends, aftertouch and note velocity are unsupported too along with SMPTE timecode timing (only metrical timing is supported, but almost every MIDI file uses this format).
