
mod artist;
mod releases;

use ::discography::Discography;
use self::artist::ArtistEndpoint;

pub struct ArtistsEndpoint<'a> {
    client: &'a Discography,
}


impl<'a> ArtistsEndpoint<'a> {
    pub fn new(client: &Discography) -> ArtistsEndpoint {
        ArtistsEndpoint { client: client }
    }

    pub fn id(&self, id: i64) -> ArtistEndpoint {
        ArtistEndpoint::new(self.client, id)
    }
}
