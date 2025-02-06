# Note pen

## Description

A music library for Rust.

This format is different from midi as it uses sheet music representations internally.
It is loosely based on the MusicXML format.

## Installation

Add the following to your `Cargo.toml` file:

```toml
note-pen = "0.3"
```

or in your shell of choice:

```sh
cargo add note-pen
```

## Features
- `serde` - Serialize and Deserialize on most types
- `musicxml` - Import/Export to MusicXML format
- `midi` - Import/Export to MIDI format
