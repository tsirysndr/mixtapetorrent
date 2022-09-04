use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mixtape {
    pub title: String,
    pub link: String,
    pub cover: String,
    pub tracks: Vec<String>,
    pub torrent: String,
    pub submitted_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dj {
    pub name: String,
    pub link: String,
}
