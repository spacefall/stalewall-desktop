mod kde;

use crate::bg::{Mode, SetMode};
use anyhow::{bail, Result};
use std::env;

pub enum DesktopEnvironment {
    Kde,
    Unsupported,
}

pub fn get_desktop_env() -> DesktopEnvironment {
    let de = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    match de.as_str() {
        "KDE" => DesktopEnvironment::Kde,
        _ => DesktopEnvironment::Unsupported,
    }
}

pub fn set(path: &str, mode: Mode, set_mode: SetMode) -> Result<()> {
    match get_desktop_env() {
        DesktopEnvironment::Kde => kde::set(path, &mode, set_mode),
        _ => bail!("DE unsupported"),
    }
}
