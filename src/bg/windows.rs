use crate::bg::{Mode, SetMode};
use anyhow::Result;
use windows::{
    core::HSTRING,
    Storage::{FileAccessMode, StorageFile},
    System::UserProfile::LockScreen,
    Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
    Win32::UI::Shell::{
        DesktopWallpaper, IDesktopWallpaper, DWPOS_CENTER, DWPOS_FILL, DWPOS_FIT, DWPOS_SPAN,
        DWPOS_STRETCH, DWPOS_TILE,
    },
};

/// Sets the desktop wallpaper from a file and/or sets the wallpaper position.
pub(crate) unsafe fn set_wallpaper(path: &str, mode: &Mode) -> Result<()> {
    CoInitialize(None).unwrap();
    let idw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)
        .expect("Couldn't initialize IDesktopWallpaper");
    let path = &HSTRING::from(path);
    let wpos = match mode {
        Mode::Center => DWPOS_CENTER,
        Mode::Tile => DWPOS_TILE,
        Mode::Stretch => DWPOS_STRETCH,
        Mode::Fit => DWPOS_FIT,
        Mode::Crop => DWPOS_FILL,
        Mode::Span => DWPOS_SPAN,
    };
    idw.SetPosition(wpos)?;
    idw.SetWallpaper(None, path)?;
    Ok(())
}

// Sets lockscreen wallpaper
pub(crate) async fn set_lockscreen(path: &str) -> Result<()> {
    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path))?.await?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.await?;
    LockScreen::SetImageStreamAsync(&stream)?.await?;
    Ok(()) 
}

// Sets both
pub(crate) async fn set_all(path: &str, mode: Mode, set_mode: SetMode) -> Result<()> {
    match set_mode {
        SetMode::Desktop => unsafe { set_wallpaper(path, &mode) },
        SetMode::Lockscreen => set_lockscreen(path).await,
        SetMode::Both => {
            let lock = set_lockscreen(path);
            unsafe { set_wallpaper(path, &mode)? };
            lock.await
        }
    }
}
