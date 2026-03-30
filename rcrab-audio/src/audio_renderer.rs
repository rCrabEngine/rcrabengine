// Audio renderer - stub implementation

use super::audio_source::AudioSource;

/// Audio renderer - stub implementation
pub struct AudioRenderer;

impl AudioRenderer {
    pub fn new() -> std::result::Result<Self, crate::Error> {
        tracing::warn!("Audio disabled - requires ALSA system libraries");
        Ok(Self)
    }

    pub fn play(&self, _source: &dyn AudioSource) -> Option<uuid::Uuid> {
        tracing::warn!("Audio disabled");
        None
    }

    pub fn stop(&self, _id: uuid::Uuid) {}

    pub fn set_master_gain(&self, _gain: f32) {}

    pub fn get_master_gain(&self) -> f32 { 1.0 }

    pub fn set_muted(&self, _muted: bool) {}

    pub fn is_muted(&self) -> bool { false }

    pub fn stop_all(&self) {}

    pub fn num_active_sounds(&self) -> usize { 0 }
}

impl Default for AudioRenderer {
    fn default() -> Self {
        Self
    }
}

/// Sound stream placeholder
pub struct SoundStream {
    name: String,
}

impl SoundStream {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
