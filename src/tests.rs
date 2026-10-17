use super::*;
use std::process::Command;
use std::time::Duration;

#[test]
fn test_config_defaults() {
    let config = Config::from_raw_args(None, None).expect("Should parse defaults");
    assert_eq!(config.bpm, 60.0);
    assert_eq!(config.beats_per_bar, 4);
}

#[test]
fn test_config_custom_valid() {
    let config = Config::from_raw_args(Some("120"), Some("3")).expect("Should parse 120 BPM 3/4");
    assert_eq!(config.bpm, 120.0);
    assert_eq!(config.beats_per_bar, 3);

    let config_float =
        Config::from_raw_args(Some("144.5"), Some("7")).expect("Should parse float BPM");
    assert_eq!(config_float.bpm, 144.5);
    assert_eq!(config_float.beats_per_bar, 7);
}

#[test]
fn test_config_invalid_bpm_zero_or_negative() {
    let err_zero = Config::from_raw_args(Some("0"), Some("4")).unwrap_err();
    assert_eq!(
        err_zero.to_string(),
        "BPM must be a positive, finite number!"
    );

    let err_neg = Config::from_raw_args(Some("-10"), Some("4")).unwrap_err();
    assert_eq!(
        err_neg.to_string(),
        "BPM must be a positive, finite number!"
    );
}

#[test]
fn test_config_invalid_bpm_nan_or_infinite() {
    let err_nan = Config::new(f64::NAN, 4).unwrap_err();
    assert_eq!(
        err_nan.to_string(),
        "BPM must be a positive, finite number!"
    );

    let err_inf = Config::new(f64::INFINITY, 4).unwrap_err();
    assert_eq!(
        err_inf.to_string(),
        "BPM must be a positive, finite number!"
    );
}

#[test]
fn test_config_invalid_beats_per_bar_zero() {
    let err_zero = Config::from_raw_args(Some("120"), Some("0")).unwrap_err();
    assert_eq!(
        err_zero.to_string(),
        "Beats Per Bar must be greater than 0!"
    );

    let err_new = Config::new(120.0, 0).unwrap_err();
    assert_eq!(err_new.to_string(), "Beats Per Bar must be greater than 0!");
}

#[test]
fn test_config_parse_failures() {
    let err_bpm = Config::from_raw_args(Some("not-a-number"), Some("4"));
    assert!(err_bpm.is_err());

    let err_beats = Config::from_raw_args(Some("120"), Some("not-an-integer"));
    assert!(err_beats.is_err());
}

#[test]
fn test_beat_duration() {
    assert_eq!(beat_duration(60.0), Duration::from_secs(1));
    assert_eq!(beat_duration(120.0), Duration::from_millis(500));
    assert_eq!(beat_duration(240.0), Duration::from_millis(250));
}

#[test]
fn test_beat_sound_accents() {
    // Downbeat (beat 0) should be high wood block accented
    let (note_0, vel_0) = beat_sound(0);
    assert_eq!(note_0, NOTE_HI_WOODBLOCK);
    assert_eq!(note_0, 76);
    assert_eq!(vel_0, VELOCITY_STRONG);
    assert_eq!(vel_0, 100);

    // Other beats should be low wood block weak
    for beat in 1..=12 {
        let (note, vel) = beat_sound(beat);
        assert_eq!(note, NOTE_LO_WOODBLOCK);
        assert_eq!(note, 77);
        assert_eq!(vel, VELOCITY_WEAK);
        assert_eq!(vel, 70);
    }
}

#[test]
fn test_midi_messages() {
    // Channel 10 Note On is 0x99
    let on_msg = note_on_message(76, 110);
    assert_eq!(on_msg, [0x99, 76, 110]);

    // Channel 10 Note Off is 0x89
    let off_msg = note_off_message(76);
    assert_eq!(off_msg, [0x89, 76, 0]);

    assert_eq!(DEFAULT_NOTE_LEN, Duration::from_millis(80));
}

#[test]
fn test_find_port_by_name() {
    let mock_ports = vec![
        "Midi Through Port-0",
        "FLUID Synth (128:0)",
        "Virtual Raw MIDI",
    ];

    let found = find_port_by_name(&mock_ports, "FLUID Synth", |p| Some(p.to_string()));
    assert_eq!(found, Some(&"FLUID Synth (128:0)"));

    let not_found = find_port_by_name(&mock_ports, "Nonexistent Synth", |p| Some(p.to_string()));
    assert_eq!(not_found, None);
}

#[test]
fn test_cli_execution_errors() {
    let bin_path =
        std::env::var("CARGO_BIN_EXE_fmetro").unwrap_or_else(|_| "target/debug/fmetro".to_string());
    if !std::path::Path::new(&bin_path).exists() {
        return;
    }

    // Test zero BPM error
    let output_zero_bpm = Command::new(&bin_path)
        .args(["0", "4"])
        .output()
        .expect("Failed to run fmetro");
    assert!(!output_zero_bpm.status.success());
    let stderr_zero = String::from_utf8_lossy(&output_zero_bpm.stderr);
    assert!(stderr_zero.contains("BPM must be a positive, finite number"));

    // Test negative BPM error
    let output_neg_bpm = Command::new(&bin_path)
        .args(["-60", "4"])
        .output()
        .expect("Failed to run fmetro");
    assert!(!output_neg_bpm.status.success());
    let stderr_neg = String::from_utf8_lossy(&output_neg_bpm.stderr);
    assert!(stderr_neg.contains("BPM must be a positive, finite number"));

    // Test zero beats_per_bar error
    let output_zero_beats = Command::new(&bin_path)
        .args(["120", "0"])
        .output()
        .expect("Failed to run fmetro");
    assert!(!output_zero_beats.status.success());
    let stderr_zero_beats = String::from_utf8_lossy(&output_zero_beats.stderr);
    assert!(stderr_zero_beats.contains("Beats Per Bar must be greater than 0!"));

    // Test invalid string argument error
    let output_invalid_str = Command::new(&bin_path)
        .args(["abc", "4"])
        .output()
        .expect("Failed to run fmetro");
    assert!(!output_invalid_str.status.success());
}
