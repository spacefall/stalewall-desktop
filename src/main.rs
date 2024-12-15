use std::fs;

mod net;
mod json_struct;

const API_URL: &str = "https://stalewall.spacefell.workers.dev";

fn main() {
    let json = net::get_api_json(API_URL).unwrap();
    println!("{}", json.url);
    net::get_image(json.url.as_str(), "test.jpg").unwrap();
    let path = fs::canonicalize("test.jpg").unwrap();
    let abs_path = path.as_path().to_str().unwrap();
    wallpaper::set_from_path(abs_path).unwrap();
}

