// Audio source - stub implementation

use crate::{Error, Result};
use std::sync::Arc;
use uuid::Uuid;

/// Audio format enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Ogg,
    Mp3,
    Flac,
    Unknown,
}

/// Audio source trait
pub trait AudioSource: Send + Sync {
    fn get_name(&self) -> &str;
    fn is_looping(&self) -> bool;
    fn set_looping(&self, looped: bool);
    fn get_volume(&self) -> f32;
    fn set_volume(&self, volume: f32);
}

/// Audio buffer - stub
pub struct AudioBuffer {
    id: Uuid,
    name: String,
    format: AudioFormat,
    duration: f32,
    looping: bool,
    volume: f32,
}

impl AudioBuffer {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let format = if path.ends_with(".wav") {
            AudioFormat::Wav
        } else if path.ends_with(".ogg") {
            AudioFormat::Ogg
        } else if path.ends_with(".mp3") {
            AudioFormat::Mp3
        } else if path.ends_with(".flac") {
            AudioFormat::Flac
        } else {
            AudioFormat::Unknown
        };

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            format,
            duration: 0.0,
            looping: false,
            volume: 1.0,
        })
    }

    pub fn get_id(&self) -> Uuid { self.id }
    pub fn get_duration(&self) -> f32 { self.duration }
    pub fn get_format(&self) -> AudioFormat { self.format }
}

impl AudioSource for AudioBuffer {
    fn get_name(&self) -> &str { &self.name }
    fn is_looping(&self) -> bool { self.looping }
    fn set_looping(&self, looped: bool) { /* stub - no mutability */ }
    fn get_volume(&self) -> f32 { self.volume }
    fn set_volume(&self, volume: f32) { /* stub - no mutability */ }
}

/// Audio stream - stub
pub struct AudioStream {
    id: Uuid,
    name: String,
    path: String,
}

impl AudioStream {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}
