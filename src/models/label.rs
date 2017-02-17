use models::misc::{Pagination,Image};

#[derive(Deserialize, Debug)]
pub struct Label {
    pub id: i64,
    pub resource_url: String,
    pub name: String,
    pub profile: Option<String>,
    pub releases_url: Option<String>,
    pub contact_info: Option<String>,
    pub uri: Option<String>,
    pub urls: Option<Vec<String>>,
    pub data_quality: Option<String>,
    pub sublabels: Option<Vec<Label>>,
    pub images: Option<Vec<Image>>,
}


#[derive(Deserialize, Debug)]
pub struct LabelRelease {
    pub title: String,
    pub id: i64,
    pub resource_url: String,
    pub year: Option<u32>,
    pub artist: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct LabelReleases {
    pub pagination: Pagination,
    pub releases: Vec<LabelRelease>,
}
