#[cfg(feature = "focus")]
mod advanced;
#[cfg(feature = "focus")]
pub use advanced::*;

#[cfg(not(feature = "focus"))]
mod simple;
#[cfg(not(feature = "focus"))]
pub use simple::*;
