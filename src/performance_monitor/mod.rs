//! Draws a performance graph of the application
//!

#[cfg(feature = "render")]
mod fps;
#[cfg(feature = "render")]
mod graph;
#[cfg(feature = "render")]
mod orthographic_uniform;
#[cfg(feature = "watch")]
pub mod watch;

#[cfg(feature = "render")]
pub use fps::Fps;
#[cfg(feature = "render")]
pub use graph::Graph;
