use crate::json_struct::Stalewall;
use anyhow::{bail, Context};
use std::fs::File;
use std::io;
use ureq::get;

pub fn get_api_json(url: &str) -> anyhow::Result<Stalewall> {
    let res = match get(url).call() {
        Ok(res) => res,
        Err(ureq::Error::Status(code, _res)) => bail!("Couldn't get the json from the api, check the parameters passed and try again. Status code: {}", code),
        Err(ureq::Error::Transport(e)) => bail!("Couldn't connect to the api, check your connection and try again.\nFull error: {}", e.message().unwrap()),
    };
    let json: Stalewall = res
        .into_json()
        .context("Couldn't parse the json from the api")?;
    Ok(json)
}

pub fn get_image(url: &str, filename: &str) -> anyhow::Result<()> {
    let mut file =
        File::create(filename).context("Couldn't create a file to download the image")?;
    let res = match get(url).call() {
        Ok(res) => res,
        Err(ureq::Error::Status(code, _res)) => bail!(
            "Couldn't get the image from the api response, try again later. Status code: {}",
            code
        ),
        Err(ureq::Error::Transport(e)) => bail!(
            "Couldn't download the image, check your connection and try again.\nFull error: {}",
            e.message().unwrap()
        ),
    };
    io::copy(&mut res.into_reader(), &mut file).context("Couldn't write the image to the file")?;
    Ok(())
}
