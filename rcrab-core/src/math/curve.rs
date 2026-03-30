// Curve and animation types

use glam::{Quat, Vec3};

/// Interpolation mode for curves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interpolation {
    Linear,
    Smooth,
    Bounce,
    Elastic,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
}

/// A keyframe with time, value, and tangent
#[derive(Debug, Clone)]
pub struct Keyframe<T> {
    pub time: f32,
    pub value: T,
    pub tangent_in: Option<T>,
    pub tangent_out: Option<T>,
}

impl<T> Keyframe<T> {
    pub fn new(time: f32, value: T) -> Self {
        Self {
            time,
            value,
            tangent_in: None,
            tangent_out: None,
        }
    }

    pub fn with_tangents(mut self, tangent_in: T, tangent_out: T) -> Self {
        self.tangent_in = Some(tangent_in);
        self.tangent_out = Some(tangent_out);
        self
    }
}

/// A track containing keyframes for a single value type
#[derive(Debug, Clone)]
pub struct Track<T> {
    pub keyframes: Vec<Keyframe<T>>,
}

impl<T> Track<T> {
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }

    pub fn with_keyframe(mut self, keyframe: Keyframe<T>) -> Self {
        self.keyframes.push(keyframe);
        self
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe<T>) {
        self.keyframes.push(keyframe);
    }

    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.keyframes.len()
    }

    /// Get the value at a given time using linear interpolation
    pub fn get_value_at(&self, time: f32) -> Option<T>
    where
        T: Clone + Copy + Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T>,
    {
        if self.keyframes.is_empty() {
            return None;
        }

        if self.keyframes.len() == 1 {
            return Some(self.keyframes[0].value);
        }

        // Find the surrounding keyframes
        let mut prev = &self.keyframes[0];
        let mut next = &self.keyframes[self.keyframes.len() - 1];

        for i in 0..self.keyframes.len() - 1 {
            if time >= self.keyframes[i].time && time <= self.keyframes[i + 1].time {
                prev = &self.keyframes[i];
                next = &self.keyframes[i + 1];
                break;
            }
        }

        // Clamp to edges
        if time <= prev.time {
            return Some(prev.value);
        }
        if time >= next.time {
            return Some(next.value);
        }

        // Linear interpolation
        let t = (time - prev.time) / (next.time - prev.time);
        Some(prev.value + (next.value - prev.value) * t)
    }
}

use std::ops::{Add, Mul, Sub};

// Type aliases for common track types
pub type Vec3Track = Track<Vec3>;
pub type FloatTrack = Track<f32>;
pub type QuatTrack = Track<Quat>;

/// An animation with a name and multiple tracks
#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub duration: f32,
    pub tracks: Vec<TrackHolder>,
}

impl Animation {
    pub fn new(name: String, duration: f32) -> Self {
        Self {
            name,
            duration,
            tracks: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: TrackHolder) {
        self.tracks.push(track);
    }
}

/// Container for different track types
#[derive(Debug, Clone)]
pub enum TrackHolder {
    Position(Vec3Track),
    Rotation(QuatTrack),
    Scale(Vec3Track),
    Custom(Vec3Track),
}

impl TrackHolder {
    pub fn get_value_at(&self, time: f32) -> Option<TrackValue> {
        match self {
            TrackHolder::Position(t) => t.get_value_at(time).map(TrackValue::Vec3),
            TrackHolder::Rotation(t) => t.get_value_at(time).map(TrackValue::Quat),
            TrackHolder::Scale(t) => t.get_value_at(time).map(TrackValue::Vec3),
            TrackHolder::Custom(t) => t.get_value_at(time).map(TrackValue::Vec3),
        }
    }
}

/// Values that can be stored in tracks
#[derive(Debug, Clone, Copy)]
pub enum TrackValue {
    Vec3(Vec3),
    Quat(Quat),
    Float(f32),
}
