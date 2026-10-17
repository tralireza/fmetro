# fmetro

[![Build](https://github.com/tralireza/fmetro/actions/workflows/build.yml/badge.svg)](https://github.com/tralireza/fmetro/actions/workflows/build.yml)

A lightweight, drift-free MIDI metronome written in Rust.

`fmetro` connects to an available MIDI port (targeting FluidSynth) and generates General MIDI Channel 10 percussion ticks using woodblock voices.

## Features

- **Drift-Free Timing**: Incremental monotonic clock scheduling avoids accumulated latency drift over long sessions.
- **Accented Downbeats**: Beat 1 plays High Wood Block (MIDI note 76 @ velocity 100); subsequent beats play Low Wood Block (MIDI note 77 @ velocity 70).
- **Flexible Meter**: Supports arbitrary tempos (floating-point BPM) and time signatures.
- **Tested**: Comprehensive test suite covering configuration parsing, interval math, and MIDI message generation.

## Requirements

### Linux
Requires ALSA development headers:

**Debian / Ubuntu**:
```bash
sudo apt-get install libasound2-dev
```

**CentOS / RHEL / Fedora**:
```bash
sudo dnf install alsa-lib-devel
```

### MIDI Synth
A running software synthesizer matching `"FLUID Synth"` (e.g., FluidSynth loaded with a General MIDI SoundFont):
```bash
fluidsynth -a alsa -m alsa_seq /usr/share/sounds/sf2/default-GM.sf2
```

## Build

```bash
cargo build --release
```

## Usage

```bash
# Default: 60 BPM, 4/4 meter
cargo run

# 120 BPM, 4/4 meter
cargo run -- 120 4

# 144 BPM, 3/4 meter
cargo run -- 144 3
```

## Testing

```bash
cargo test
```
