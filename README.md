# stalewall-desktop

A desktop application that uses stalewall, an api that provides random wallpapers.  
This application gets a random wallpaper and sets it as the desktop and/or lockscreen wallpaper.  
It currently supports:

- Windows
- KDE

More will be added in the future  
This project requires a working [stalewall](https://github.com/spacefall/stalewall) api url

## Usage
```
Usage: stalewall-desktop [OPTIONS] -a <MODE> [WIDTH] [HEIGHT]

Arguments:
  [WIDTH]   Sets width
  [HEIGHT]  Sets height

Options:
  -p <PROVIDERS>      Sets providers to use, see stalewall readme for more info
  -u <URL>            Change api url, default is <https://stalewall.spacefell.workers.dev>
  -a <MODE>           Selects where to apply wallpaper [possible values: desktop, lockscreen, both]
  -m <MODE>           Change wallpaper mode [possible values: center, crop, fit, span, stretch, tile]
  -h, --help          Print help
  -V, --version       Print version
```
