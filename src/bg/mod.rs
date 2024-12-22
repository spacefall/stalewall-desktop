#[cfg(windows)]
mod windows;

#[cfg(windows)]
use futures_lite::future::block_on;

use anyhow::Result;

pub enum Mode {
    Center,
    Crop,
    Fit,
    Span,
    Stretch,
    Tile,
}

pub enum SetMode {
    Desktop,
    Lockscreen,
    Both,
}

#[cfg(windows)]
pub fn set(path: &str, mode: Mode, set_mode: SetMode) -> Result<()> {
    block_on(windows::set_all(path, mode, set_mode))
}