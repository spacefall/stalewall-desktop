use crate::bg::{Mode, SetMode};
use crate::json_struct::Stalewall;
use clap::Parser;
use std::env;

mod bg;
mod json_struct;
mod net;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets request width
    #[arg(requires = "height")]
    width: Option<u16>,

    /// Sets request height
    #[arg(requires = "width")]
    height: Option<u16>,

    /// Sets request providers
    #[arg(short)]
    providers: Option<String>,
}

const API_URL: &str = "https://stalewall.spacefell.workers.dev";

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();
    let mut params: String = String::new();

    if cli.width.is_some() && cli.height.is_some() {
        let w = cli.width.unwrap();
        let h = cli.height.unwrap();
        params = format!("?res={}x{}", w, h);
    }

    if let Some(p) = cli.providers.as_deref() {
        let separator = if params.is_empty() { "?" } else { "&" };
        params = format!("{}{}prov={}", params, separator, p);
    }

    // Make the request
    let json = net::get_api_json(format!("{API_URL}/{params}").as_str()).expect("Wrong arguments passed, api request failed");

    // Print the response
    // I'm sure this is memory efficient
    print_resp(json.clone());

    // Download image to temp files
    println!("Downloading image...");
    let temp_image_path = env::temp_dir().join("stalewall_current.jpg");
    let save_path = temp_image_path.to_str().unwrap();
    net::get_image(json.url.as_str(), save_path).expect("Couldn't download the image");

    // Set wallpaper
    println!("Setting image as wallpaper");
    bg::set(save_path, Mode::Crop, SetMode::Both).unwrap()
}

/// Prints a Stalewall struct, it's here just to make the code cleaner (slightly) 
fn print_resp(json: Stalewall) {
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
}
