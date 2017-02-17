//! Labels API
//!
//! https://www.discogs.com/developers/#page:database,header:database-label
//!

mod label;
mod releases;

use ::discography::Discography;
use self::label::LabelEndpoint;

pub struct LabelsEndpoint<'a> {
    client: &'a Discography,
}


impl<'a> LabelsEndpoint<'a> {
    pub fn new(client: &Discography) -> LabelsEndpoint {
        LabelsEndpoint { client: client }
    }

    pub fn id(&self, id: i64) -> LabelEndpoint {
        LabelEndpoint::new(self.client, id)
    }
}
