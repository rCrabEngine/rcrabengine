// App State management

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Application state lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initialized,
    Running,
    Paused,
    Stopped,
}

/// Base trait for application states
pub trait AppState: Send + Sync {
    /// Get state ID
    fn get_id(&self) -> &str;

    /// Called when state is attached
    fn initialize(&self, state_manager: &AppStateManager, app: &dyn crate::app::Application);

    /// Called when state is detached (cleanup)
    fn cleanup(&self);

    /// Called before render
    fn update(&self, tpf: f32);

    /// Called on render
    fn render(&self, render_manager: &mut dyn RenderableManager);

    /// Check if state is enabled
    fn is_enabled(&self) -> bool;

    /// Set enabled state
    fn set_enabled(&self, enabled: bool);

    /// Called when state becomes active
    fn on_state_active(&self);

    /// Called when state becomes inactive
    fn on_state_inactive(&self);

    /// Called on pause
    fn on_state_paused(&self);

    /// Called on resume
    fn on_state_resumed(&self);
}

/// Placeholder for render manager
pub trait RenderableManager: Send + Sync {}

/// Application state manager
pub struct AppStateManager {
    id: Uuid,
    states: RwLock<HashMap<String, Arc<dyn AppState>>>,
    state_list: RwLock<Vec<Arc<dyn AppState>>>,
    app: RwLock<Option<Arc<dyn crate::app::Application>>>,
}

impl AppStateManager {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            states: RwLock::new(HashMap::new()),
            state_list: RwLock::new(Vec::new()),
            app: RwLock::new(None),
        }
    }

    /// Attach an application reference
    pub fn set_application(&self, app: Arc<dyn crate::app::Application>) {
        *self.app.write() = Some(app);
    }

    /// Attach a state
    pub fn attach(&self, state: Arc<dyn AppState>) {
        let id = state.get_id().to_string();
        let mut states = self.states.write();
        let mut state_list = self.state_list.write();

        if !states.contains_key(&id) {
            states.insert(id.clone(), state.clone());
            state_list.push(state.clone());

            // Initialize the state
            if let Some(app) = self.app.read().as_ref() {
                state.initialize(self, app.as_ref());
            }
        }
    }

    /// Detach a state by ID
    pub fn detach(&self, id: &str) {
        let mut states = self.states.write();
        let mut state_list = self.state_list.write();

        if let Some(state) = states.remove(id) {
            state.cleanup();
            state_list.retain(|s| s.get_id() != id);
        }
    }

    /// Detach a state
    pub fn detach_state(&self, state: &dyn AppState) {
        self.detach(state.get_id());
    }

    /// Get a state by ID
    pub fn get_state<T: AppState + 'static>(&self, id: &str) -> Option<Arc<T>> {
        self.states
            .read()
            .get(id)
            .and_then(|s| s.clone().downcast::<T>())
    }

    /// Get all states
    pub fn get_states(&self) -> Vec<Arc<dyn AppState>> {
        self.state_list.read().clone()
    }

    /// Check if has state
    pub fn has_state(&self, id: &str) -> bool {
        self.states.read().contains_key(id)
    }

    /// Update all states
    pub fn update(&self, tpf: f32) {
        let states = self.state_list.read();
        for state in states.iter() {
            if state.is_enabled() {
                state.update(tpf);
            }
        }
    }

    /// Detach all states
    pub fn detach_all(&self) {
        let mut states = self.states.write();
        let mut state_list = self.state_list.write();

        for state in state_list.iter() {
            state.cleanup();
        }

        states.clear();
        state_list.clear();
    }
}

impl Default for AppStateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper trait for downcasting
pub trait Downcast {
    fn downcast<T: AppState + 'static>(&self) -> Option<Arc<T>>;
}

impl Downcast for Arc<dyn AppState> {
    fn downcast<T: AppState + 'static>(&self) -> Option<Arc<T>> {
        // This is a simplified version - in practice you'd use TypeId
        // For now, we'll use Any if needed
        None
    }
}
