use std::env;
use crate::bg::{Mode, SetMode};

mod net;
mod json_struct;
mod bg;

const API_URL: &str = "https://stalewall.spacefell.workers.dev";

fn main() {
    let json = net::get_api_json(API_URL).unwrap();
    // Print the response
    println!("Got response:");
    println!("Provider: {}", json.provider);
    println!("URL: {}", json.url);
    println!("Copyright: {}", json.info.credits.copyright);
    if json.info.credits.urls.is_some() {
        println!("Links:");
        let creds = json.info.credits.urls.unwrap();
        if creds.author.is_some() {
            println!("Author: {}", creds.author.unwrap());
        }
        if creds.image.is_some() {
            println!("Image: {}", creds.image.unwrap());
        }
    }
    if json.info.desc.is_some() {
        println!("Descriptions:");
        let desc = json.info.desc.unwrap();
        if desc.title.is_some() {
            println!("Title: {}", desc.title.unwrap());
        }
        if desc.short.is_some() {
            println!("Short: {}", desc.short.unwrap());
        }
        if desc.long.is_some() {
            println!("Long: {}", desc.long.unwrap());
        }
    }

    // Download image to temp files
    let temp_image_path = env::temp_dir().join("stalewall_current.jpg");
    let save_path = temp_image_path.to_str().unwrap();
    net::get_image(json.url.as_str(), save_path).expect("Couldn't download the image");

    // Set wallpaper
    bg::set(save_path, Mode::Crop, SetMode::Both).unwrap()
}

