use models::artist::Artist;


#[derive(Deserialize, Debug)]
pub struct Release {
    pub title: String,
    pub id: u32,
    pub released: String,
    pub released_formatted: String,
    pub resource_url: String,
    pub date_added: String,
    pub date_changed: String,
    pub uri: String,
    pub year: u32,
    pub artists: Vec<Artist>,
    pub status: String,
}
