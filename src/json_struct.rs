use serde::Deserialize;

#[derive(Deserialize)]
pub struct Stalewall {
    pub provider: String,
    pub url: String,
    pub info: Info,
}

#[derive(Deserialize)]
pub struct Info {
    pub desc: Option<Description>,
    pub credits: Credits
}

#[derive(Deserialize)]
pub struct Description {
    pub title: Option<String>,
    pub short: Option<String>,
    pub long: Option<String>,
}

#[derive(Deserialize)]
pub struct Credits {
    pub copyright: String,
    pub urls: Option<Urls>
}

#[derive(Deserialize)]
pub struct Urls {
    pub author: Option<String>,
    pub image: Option<String>,
}