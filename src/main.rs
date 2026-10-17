use fmetro::{
    Config, DEFAULT_NOTE_LEN, beat_duration, beat_sound, find_port_by_name, note_off_message,
    note_on_message,
};
use midir::MidiOutput;
use std::{env, thread, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_raw_args(
        args.get(1).map(|s| s.as_str()),
        args.get(2).map(|s| s.as_str()),
    )?;

    let midi_out = MidiOutput::new("fmetro")?;
    let ports = midi_out.ports();

    // finds FLUID Synth "128:0"
    let port = find_port_by_name(&ports, "FLUID Synth", |p| midi_out.port_name(p).ok())
        //.or_else(|| ports.first())
        .expect("No ALSA MIDI ports - is FluidSynth running?");

    println!(
        "fmetro -> {} @ {} BPM, {}/4",
        midi_out.port_name(port)?,
        config.bpm,
        config.beats_per_bar
    );
    let mut conn = midi_out.connect(port, "fmetro-out")?;

    let beat_dur = beat_duration(config.bpm);
    let note_len = DEFAULT_NOTE_LEN;
    let mut next_tick = Instant::now() + beat_dur;
    let mut beat = 0;

    loop {
        let now = Instant::now();
        if now < next_tick {
            thread::sleep(next_tick - now);
        }

        let (note, vel) = beat_sound(beat);

        conn.send(&note_on_message(note, vel))?;
        thread::sleep(note_len);
        conn.send(&note_off_message(note))?;

        beat = (beat + 1) % config.beats_per_bar;
        next_tick += beat_dur;
    }
}
