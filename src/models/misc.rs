use IntoParam;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub per_page: i16,
    pub page: i16,
    pub items: i64,
    pub pages: i16,
    pub urls: PaginationUrls,
}

#[derive(Deserialize, Debug)]
pub struct PaginationUrls {
    pub next: Option<String>,
    pub last: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct Image {}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    #[serde(rename="release")]
    Release,
    #[serde(rename="master")]
    Master,
    #[serde(rename="artist")]
    Artist,
    #[serde(rename="label")]
    Label,
}


impl IntoParam for Type {
    fn to_param(&self) -> String {
        let matched = match *self {
            Type::Artist => "artist",
            Type::Master => "master",
            Type::Release => "release",
            Type::Label => "label",
        };

        String::from(matched)
    }
}
