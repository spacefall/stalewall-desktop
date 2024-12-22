use std::fs::File;
use std::io;
use crate::json_struct::Stalewall;

pub fn get_api_json(url: &str) -> Result<Stalewall, Box<dyn std::error::Error>> {
    let res = ureq::get(url).call()?;
    let json_res: Stalewall = res.into_json()?;
    Ok(json_res)
}

pub fn get_image(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    let res = ureq::get(url).call()?;
    io::copy(&mut res.into_reader(), &mut file)?;
    Ok(())
}