use crate::default_application::state::State;




/// Events delivered to the winit loop from outside a `WindowEvent`.
pub enum UserEvent {
    /// The async setup finished; carries the initialized `State`. Boxed to keep
    /// the event small (`State` is large).
    Initialized(Box<State>),
}