use std::env;
use crate::bg::Mode;

mod net;
mod json_struct;
mod bg;

const API_URL: &str = "https://stalewall.spacefell.workers.dev";

fn main() {
    let json = net::get_api_json(API_URL).unwrap();
    println!("{}", json.url);
    let tmppathbuf = env::temp_dir().join("stalewall_current.jpg");
    let tmppath = tmppathbuf.to_str().unwrap();
    net::get_image(json.url.as_str(), tmppath).unwrap();
    bg::set_wallpaper(Some(tmppath), Some(Mode::Crop)).expect("Couldn't set desktop wallpaper");
    bg::set_lockscreen(tmppath).expect("Couldn't set lockscreen wallpaper");
}

