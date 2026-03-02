//! Handles focus between UI elements
//! Uses Bevy's built-in input_focus (simple). The optional `focus` feature and
//! bevy-alt-ui-navigation-lite were removed for Bevy 0.18 compatibility.
mod simple;
pub use simple::*;
