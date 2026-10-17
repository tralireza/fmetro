use std::time::Duration;

pub const NOTE_HI_WOODBLOCK: u8 = 76;
pub const NOTE_LO_WOODBLOCK: u8 = 77;
pub const VELOCITY_STRONG: u8 = 100;
pub const VELOCITY_WEAK: u8 = 70;
pub const DEFAULT_NOTE_LEN: Duration = Duration::from_millis(80);

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub bpm: f64,
    pub beats_per_bar: u32,
}

impl Config {
    pub fn new(bpm: f64, beats_per_bar: u32) -> Result<Self, Box<dyn std::error::Error>> {
        if bpm <= 0.0 || !bpm.is_finite() {
            return Err("BPM must be a positive, finite number!".into());
        }
        if beats_per_bar == 0 {
            return Err("Beats Per Bar must be greater than 0!".into());
        }
        Ok(Self { bpm, beats_per_bar })
    }

    pub fn from_raw_args(
        raw_bpm: Option<&str>,
        raw_beats: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let bpm: f64 = raw_bpm.unwrap_or("60").parse()?;
        let beats_per_bar: u32 = raw_beats.unwrap_or("4").parse()?;
        Self::new(bpm, beats_per_bar)
    }
}

pub fn beat_duration(bpm: f64) -> Duration {
    Duration::from_secs_f64(60.0 / bpm)
}

pub fn beat_sound(beat: u32) -> (u8, u8) {
    if beat == 0 {
        (NOTE_HI_WOODBLOCK, VELOCITY_STRONG)
    } else {
        (NOTE_LO_WOODBLOCK, VELOCITY_WEAK)
    }
}

pub fn note_on_message(note: u8, vel: u8) -> [u8; 3] {
    [0x99, note, vel] // 0x99 = note_on ch10
}

pub fn note_off_message(note: u8) -> [u8; 3] {
    [0x89, note, 0] // 0x89 = note_off ch10
}

pub fn find_port_by_name<'a, T, F>(ports: &'a [T], target: &str, mut get_name: F) -> Option<&'a T>
where
    F: FnMut(&'a T) -> Option<String>,
{
    ports.iter().find(|p| {
        let name = get_name(p).unwrap_or_default();
        name.contains(target)
    })
}

#[cfg(test)]
mod tests;
