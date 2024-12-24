use crate::bg::{Mode, SetMode};
use anyhow::Result;
use std::process::Command;

// Sets the desktop wallpaper
fn set_desktop(path: &str, mode: &Mode) -> Result<()> {
    // Parses selected wallaper mode into the correct number for kde
    let fill_mode = match mode {
        Mode::Stretch => 0,
        Mode::Fit => 1,
        // only 5 mode are available on kde so that's why span and crop are together
        Mode::Span | Mode::Crop => 2,
        Mode::Tile => 3,
        Mode::Center => 6,
    };

    // Script to eval, it's ugly, but I don't want to break it with whitespace
    let script = &format!(
        r#"
for (const desktop of desktops()) {{
    desktop.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"];
    desktop.writeConfig("Image", "");
    desktop.writeConfig("Image", "{path}");
    desktop.writeConfig("FillMode", {fill_mode});
    desktop.reloadConfig();
}}"#
    );

    // Run the command
    Command::new("qdbus")
        .args([
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            script,
        ])
        .output()?;
    Ok(())
}

// Sets the lockscreen wallpaper
fn set_lockscreen(path: &str) -> Result<()> {
    // Run the command to set the lockscreen wallpaper
    Command::new("kwriteconfig5")
        .args([
            "--file",
            "kscreenlockerrc",
            "--group",
            "Greeter",
            "--group",
            "Wallpaper",
            "--group",
            "org.kde.image",
            "--group",
            "General",
            "--key",
            "Image",
            format!("file://{path}").as_str(),
        ])
        .output()?;
    Ok(())
}

// Sets the desktop and or lockscreen wallpaper according to SetMode
pub(crate) fn set(path: &str, mode: &Mode, set_mode: SetMode) -> Result<()> {
    match set_mode {
        SetMode::Desktop => set_desktop(path, mode),
        SetMode::Lockscreen => set_lockscreen(path),
        SetMode::Both => {
            set_desktop(path, mode)?;
            set_lockscreen(path)
        }
    }
}
