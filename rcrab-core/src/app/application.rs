// Application framework

use crate::app::{AppSettings, AppStateManager};
use crate::math::Vec3;
use crate::scene::{Camera, Node};
use parking_lot::RwLock;
use std::sync::Arc;

/// The main application trait - defines the interface for game applications
pub trait Application: Send + Sync {
    /// Get application settings
    fn get_settings(&self) -> Arc<AppSettings>;

    /// Get the root node
    fn get_root_node(&self) -> Arc<Node>;

    /// Get the gui node (for UI elements)
    fn get_gui_node(&self) -> Option<Arc<Node>>;

    /// Get the camera
    fn get_camera(&self) -> Option<Arc<Camera>>;

    /// Get the state manager
    fn get_state_manager(&self) -> Option<Arc<AppStateManager>>;

    /// Get the asset manager
    fn get_asset_manager(&self) -> Option<Arc<dyn AssetManager>>;

    /// Check if running
    fn is_running(&self) -> bool;

    /// Start the application
    fn start(&self);

    /// Stop the application
    fn stop(&self);

    /// Request cleanup
    fn request_cleanup(&self);

    /// Get frame rate
    fn get_frame_rate(&self) -> f32;

    /// Get frame time
    fn get_frame_time(&self) -> f32;

    /// Get time since start
    fn get_time_since_start(&self) -> f32;
}

/// Asset manager trait
pub trait AssetManager: Send + Sync {
    /// Load an asset by name and extension
    fn load(&self, name: &str, extension: &str) -> Option<Box<dyn Asset>>;

    /// Register a locator
    fn register_locator(&self, locator: Box<dyn AssetLocator>);

    /// Register a loader
    fn register_loader(&self, loader: Box<dyn AssetLoader>, extensions: &[&str]);
}

/// Asset key for loading
pub trait AssetKey: Send + Sync {
    fn get_name(&self) -> &str;
    fn get_extension(&self) -> &str;
}

/// Asset locator
pub trait AssetLocator: Send + Sync {
    fn locate(&self, name: &str) -> Option<String>;
}

/// Asset loader
pub trait AssetLoader: Send + Sync {
    fn load(&self, name: &str, data: &[u8]) -> Option<Box<dyn Asset>>;
    fn get_extensions(&self) -> Vec<&str>;
}

/// Asset trait
pub trait Asset: Send + Sync {}

/// Simple application state
pub struct ApplicationState {
    pub paused: bool,
    pub running: bool,
    pub frame_rate: f32,
    pub frame_time: f32,
    pub time_since_start: f32,
    pub context: Option<Box<dyn ApplicationContext>>,
}

pub trait ApplicationContext: Send + Sync {}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            paused: false,
            running: false,
            frame_rate: 0.0,
            frame_time: 0.0,
            time_since_start: 0.0,
            context: None,
        }
    }
}

impl ApplicationState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// SimpleApplication - basic application implementation (stub)
pub struct SimpleApplication;

impl SimpleApplication {
    pub fn new() -> Self {
        Self
    }
}
