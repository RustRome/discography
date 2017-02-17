use models::misc::{Image, Pagination};

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub id: u32,
    pub name: String,
    pub resource_url: String,
    pub tracks: Option<String>,
    pub uri: Option<String>,
    pub releases_url: Option<String>,
    pub join: Option<String>,
    pub role: Option<String>,
    pub anv: Option<String>,
    pub active: Option<bool>,
    pub namevariations: Option<Vec<String>>,
    pub urls: Option<Vec<String>>,
    pub images: Option<Vec<Image>>,
    pub profile: Option<String>,
    pub data_quality: Option<String>,
    pub realname: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct ArtistRelease {
    pub title: String,
    pub id: i64,
    pub resource_url: String,
    pub year: Option<u32>,
    pub artist: String,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ArtistReleases {
    pub pagination: Pagination,
    pub releases: Vec<ArtistRelease>,
}
