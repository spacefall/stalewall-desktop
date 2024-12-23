#[cfg(windows)]
mod windows;

#[cfg(windows)]
use futures_lite::future::block_on;

use anyhow::Result;

#[derive(clap::ValueEnum, Clone)]
pub enum Mode {
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}

#[derive(clap::ValueEnum, Clone)]
pub enum SetMode {
    Desktop,
    Lockscreen,
    Both,
}

#[cfg(windows)]
pub fn set(path: &str, mode: Mode, set_mode: SetMode) -> Result<()> {
    block_on(windows::set_all(path, mode, set_mode))
}

#[cfg(windows)]
pub fn supported() -> bool {
    true
}

#[cfg(not(windows))]
pub fn supported() -> bool {
    false
}
