//! Database API
//!
//! https://www.discogs.com/developers/#page:database
//!
use ::discography::Discography;

mod labels;
mod artists;
mod search;

pub use self::labels::LabelsEndpoint;
pub use self::artists::ArtistsEndpoint;
pub use self::search::SearchEndpoint;

pub struct Database<'a> {
    client: &'a Discography,
}


impl<'a> Database<'a> {
    pub fn new(client: &Discography) -> Database {
        Database { client: client }
    }


    pub fn labels(&self) -> LabelsEndpoint {
        LabelsEndpoint::new(self.client)
    }
    pub fn artists(&self) -> ArtistsEndpoint {
        ArtistsEndpoint::new(self.client)
    }

    pub fn search(&self) -> SearchEndpoint {
        SearchEndpoint::new(self.client)
    }
}
