// windows
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::*;

pub enum Mode {
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}