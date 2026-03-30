// Asset management

use std::sync::Arc;

/// Asset key for loading
pub struct AssetKey {
    name: String,
    extension: String,
}

impl AssetKey {
    pub fn new(name: &str) -> Self {
        let extension = std::path::Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        Self {
            name: name.to_string(),
            extension,
        }
    }

    pub fn with_extension(name: &str, extension: &str) -> Self {
        Self {
            name: name.to_string(),
            extension: extension.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_extension(&self) -> &str {
        &self.extension
    }
}

/// Texture key
pub struct TextureKey {
    name: String,
    generate_mipmaps: bool,
    anisotropy: f32,
}

impl TextureKey {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            generate_mipmaps: true,
            anisotropy: 1.0,
        }
    }

    pub fn with_mipmaps(mut self, generate: bool) -> Self {
        self.generate_mipmaps = generate;
        self
    }

    pub fn with_anisotropy(mut self, value: f32) -> Self {
        self.anisotropy = value;
        self
    }
}

/// Model key
pub struct ModelKey {
    name: String,
}

impl ModelKey {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

/// Audio key
pub struct AudioKey {
    name: String,
    stream: bool,
}

impl AudioKey {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stream: false,
        }
    }

    pub fn streaming(mut self) -> Self {
        self.stream = true;
        self
    }
}

/// Material key
pub struct MaterialKey {
    name: String,
}

impl MaterialKey {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
