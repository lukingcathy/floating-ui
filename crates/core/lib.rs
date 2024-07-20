//! Rust port of [Floating UI](https://floating-ui.com/).
//!
//! This is the platform-agnostic core of Floating UI, exposing the main [`compute_position`][`crate::compute_position::compute_position()`] function but no platform interface logic.
//!
//! See [@floating-ui/core](https://www.npmjs.com/package/@floating-ui/core) for the original package.

pub use compute_coords_from_placement::*;
pub use compute_position::*;
pub use detect_overflow::*;
pub use types::*;

mod compute_coords_from_placement;
mod compute_position;
mod detect_overflow;
pub mod middleware;
mod types;
