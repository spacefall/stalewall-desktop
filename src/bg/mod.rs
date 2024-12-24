#[cfg(windows)]
mod windows;

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;

#[cfg(windows)]
use futures_lite::future::block_on;

#[cfg(not(any(windows, all(unix, not(target_os = "macos")))))]
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
    block_on(windows::set(path, mode, set_mode))
}

#[cfg(windows)]
pub fn supported() -> bool {
    true
}

#[cfg(all(unix, not(target_os = "macos")))]
pub use linux::*;

#[cfg(all(unix, not(target_os = "macos")))]
pub fn supported() -> bool {
    !matches!(
        linux::get_desktop_env(),
        linux::DesktopEnvironment::Unsupported
    )
}

#[cfg(not(any(windows, all(unix, not(target_os = "macos")))))]
pub fn supported() -> bool {
    false
}
