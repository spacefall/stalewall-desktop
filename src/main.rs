use crate::bg::{set, Mode, SetMode};
use crate::json_struct::Stalewall;
use clap::Parser;
use std::env;

mod bg;
mod json_struct;
mod net;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets width
    #[arg(requires = "height")]
    width: Option<u16>,

    /// Sets height
    #[arg(requires = "width")]
    height: Option<u16>,

    /// Sets providers to use, see stalewall readme for more info
    #[arg(short)]
    providers: Option<String>,

    /// Change api url, default is https://stalewall.spacefell.workers.dev
    #[arg(short)]
    url: Option<String>,

    /// Selects where to apply wallpaper
    #[arg(short, value_name = "MODE")]
    apply: SetMode,

    /// Change wallpaper mode
    #[arg(short)]
    mode: Option<Mode>,
}

const API_URL: &str = "https://stalewall.spacefell.workers.dev";

fn main() {
    if !bg::supported() {
        eprintln!("Your OS is not supported, sorry!");
        std::process::exit(1);
    }
    // Parse CLI arguments
    let cli = Cli::parse();
    let mut api_request = cli.url.unwrap_or(format!("{API_URL}/").to_string());

    if cli.width.is_some() && cli.height.is_some() {
        let w = cli.width.unwrap();
        let h = cli.height.unwrap();
        api_request.push_str(&format!("?res={}x{}", w, h));
    }

    if let Some(p) = cli.providers.as_deref() {
        api_request.push(if api_request.ends_with("/") { '?' } else { '&' });
        api_request.push_str(&format!("prov={}", p));
    }

    let mode = cli.mode.unwrap_or(Mode::Crop);

    // Make the request
    let json = net::get_api_json(api_request.as_str()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    // Print the response
    // I'm sure this is memory efficient
    print_resp(json.clone());

    // Download image to temp files
    println!("Downloading image...");
    let temp_image_path = env::temp_dir().join("stalewall_current.jpg");
    let save_path = temp_image_path.to_str().unwrap();
    net::get_image(json.url.as_str(), save_path).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    // Set wallpaper
    println!("Setting image as wallpaper");
    set(save_path, mode, cli.apply).expect("Couldn't set the wallpaper");
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
