// Application settings

use parking_lot::RwLock;
use std::collections::HashMap;

/// Audio settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioRenderer {
    #[default]
    LWJGL,
    JOAL,
    Android,
    None,
}

/// Renderer type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Renderer {
    OpenGL2,
    OpenGL3,
    OpenGLES,
    Vulkan,
    Direct3D,
    #[default]
    Auto,
}

/// Context type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextType {
    Windowed,
    Fullscreen,
    Headless,
}

/// Application settings
pub struct AppSettings {
    values: RwLock<HashMap<String, SettingValue>>,
}

#[derive(Debug, Clone)]
pub enum SettingValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

impl AppSettings {
    pub fn new() -> Self {
        let mut settings = Self {
            values: RwLock::new(HashMap::new()),
        };
        settings.set_defaults();
        settings
    }

    fn set_defaults(&mut self) {
        // Window settings
        self.set("Width", 1280);
        self.set("Height", 720);
        self.set("DepthBits", 24);
        self.set("AlphaBits", 0);
        self.set("StencilBits", 0);
        self.set("Samples", 0); // Anti-aliasing
        self.set("VSync", true);
        self.set("FrameRate", 60);
        self.set("Title", "rCrabEngine Application".to_string());

        // Renderer settings
        self.set("Renderer", Renderer::Auto);
        self.set("GraphicsDebug", false);
        self.set("PackedGeometry", true);
        self.set("Instancing", true);

        // Audio settings
        self.set("AudioRenderer", AudioRenderer::None);
        self.set("AudioFrequency", 44100);
        self.set("AudioChannels", 2);
        self.set("AudioBuffersize", 512);

        // Physics settings
        self.set("PhysicsThreads", 2);

        // Input settings
        self.set("MouseGrab", false);
        self.set("EmulateTouch", false);

        // Language/Locale
        self.set("Language", "en".to_string());

        // Frustum culling
        self.set("FrustumCulling", true);
    }

    /// Set a setting value
    pub fn set<T: Into<SettingValue>>(&self, key: &str, value: T) {
        self.values.write().insert(key.to_string(), value.into());
    }

    /// Get a setting value
    pub fn get<T: TryFrom<SettingValue>>(&self, key: &str) -> Option<T> {
        self.values
            .read()
            .get(key)
            .and_then(|v| T::try_from(v.clone()).ok())
    }

    /// Get or default
    pub fn get_or<T: TryFrom<SettingValue> + Default>(&self, key: &str, default: T) -> T {
        self.get(key).unwrap_or(default)
    }

    /// Check if a setting exists
    pub fn contains(&self, key: &str) -> bool {
        self.values.read().contains_key(key)
    }

    /// Get width
    pub fn get_width(&self) -> i32 {
        self.get_or("Width", 1280)
    }

    /// Get height
    pub fn get_height(&self) -> i32 {
        self.get_or("Height", 720)
    }

    /// Get vsync
    pub fn is_vsync(&self) -> bool {
        self.get_or("VSync", true)
    }

    /// Get fullscreen
    pub fn is_fullscreen(&self) -> bool {
        self.get_or("Fullscreen", false)
    }

    /// Set fullscreen
    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.set("Fullscreen", fullscreen);
    }

    /// Get title
    pub fn get_title(&self) -> String {
        self.get_or("Title", "rCrabEngine".to_string())
    }

    /// Get renderer
    pub fn get_renderer(&self) -> Renderer {
        self.get_or("Renderer", Renderer::Auto)
    }

    /// Get audio renderer
    pub fn get_audio_renderer(&self) -> AudioRenderer {
        self.get_or("AudioRenderer", AudioRenderer::None)
    }

    /// Copy settings from another
    pub fn copy_from(&self, other: &AppSettings) {
        let values = other.values.read();
        *self.values.write() = values.clone();
    }

    /// Save to JSON (returns string)
    pub fn to_json(&self) -> String {
        // Simplified - just return a placeholder
        "{}".to_string()
    }

    /// Load from JSON
    pub fn from_json(&self, json: &str) {
        // Simplified
        let _ = json;
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self::new()
    }
}

// Implement From for setting values
impl From<bool> for SettingValue {
    fn from(v: bool) -> Self { SettingValue::Bool(v) }
}

impl From<i32> for SettingValue {
    fn from(v: i32) -> Self { SettingValue::Int(v) }
}

impl From<f32> for SettingValue {
    fn from(v: f32) -> Self { SettingValue::Float(v) }
}

impl From<String> for SettingValue {
    fn from(v: String) -> Self { SettingValue::String(v) }
}

impl From<&str> for SettingValue {
    fn from(v: &str) -> Self { SettingValue::String(v.to_string()) }
}

impl From<Renderer> for SettingValue {
    fn from(v: Renderer) -> Self { SettingValue::Int(v as i32) }
}

impl From<AudioRenderer> for SettingValue {
    fn from(v: AudioRenderer) -> Self { SettingValue::Int(v as i32) }
}

// Implement From setting values
impl From<&SettingValue> for bool {
    fn from(v: &SettingValue) -> Self {
        match v { SettingValue::Bool(b) => *b, _ => false }
    }
}

impl From<&SettingValue> for i32 {
    fn from(v: &SettingValue) -> Self {
        match v { SettingValue::Int(i) => *i, _ => 0 }
    }
}

impl From<&SettingValue> for f32 {
    fn from(v: &SettingValue) -> Self {
        match v { SettingValue::Float(f) => *f, _ => 0.0 }
    }
}

impl From<&SettingValue> for String {
    fn from(v: &SettingValue) -> Self {
        match v { SettingValue::String(s) => s.clone(), _ => String::new() }
    }
}

impl From<&SettingValue> for Renderer {
    fn from(v: &SettingValue) -> Self {
        match v { SettingValue::Int(i) => Renderer::from_i32(*i), _ => Renderer::Auto }
    }
}

impl Renderer {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Renderer::OpenGL2,
            1 => Renderer::OpenGL3,
            2 => Renderer::OpenGLES,
            3 => Renderer::Vulkan,
            4 => Renderer::Direct3D,
            _ => Renderer::Auto,
        }
    }
}

impl From<&SettingValue> for AudioRenderer {
    fn from(v: &SettingValue) -> Self {
        match v { SettingValue::Int(i) => AudioRenderer::from_i32(*i), _ => AudioRenderer::None }
    }
}

impl AudioRenderer {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => AudioRenderer::LWJGL,
            1 => AudioRenderer::JOAL,
            2 => AudioRenderer::Android,
            _ => AudioRenderer::None,
        }
    }
}

// TryFrom implementations for get<T>
use std::convert::TryFrom;

impl TryFrom<SettingValue> for bool {
    type Error = ();
    fn try_from(v: SettingValue) -> Result<Self, Self::Error> {
        match v { SettingValue::Bool(b) => Ok(b), _ => Err(()) }
    }
}

impl TryFrom<SettingValue> for i32 {
    type Error = ();
    fn try_from(v: SettingValue) -> Result<Self, Self::Error> {
        match v { SettingValue::Int(i) => Ok(i), _ => Err(()) }
    }
}

impl TryFrom<SettingValue> for f32 {
    type Error = ();
    fn try_from(v: SettingValue) -> Result<Self, Self::Error> {
        match v { SettingValue::Float(f) => Ok(f), _ => Err(()) }
    }
}

impl TryFrom<SettingValue> for String {
    type Error = ();
    fn try_from(v: SettingValue) -> Result<Self, Self::Error> {
        match v { SettingValue::String(s) => Ok(s), _ => Err(()) }
    }
}

impl TryFrom<SettingValue> for Renderer {
    type Error = ();
    fn try_from(v: SettingValue) -> Result<Self, Self::Error> {
        match v { SettingValue::Int(i) => Ok(Renderer::from_i32(i)), _ => Err(()) }
    }
}

impl TryFrom<SettingValue> for AudioRenderer {
    type Error = ();
    fn try_from(v: SettingValue) -> Result<Self, Self::Error> {
        match v { SettingValue::Int(i) => Ok(AudioRenderer::from_i32(i)), _ => Err(()) }
    }
}
