use ::discography::Discography;
use {Query, InternalEndpoint, Endpoint, DiscographyResult};
use models::artist::Artist;
use database::artists::releases::ArtistReleasesEndpoint;

pub struct ArtistEndpoint<'a> {
    client: &'a Discography,
    id: i64,
}


impl<'a> ArtistEndpoint<'a> {
    pub fn new(client: &Discography, id: i64) -> ArtistEndpoint {
        ArtistEndpoint {
            id: id,
            client: client,
        }
    }

    pub fn releases(&self) -> ArtistReleasesEndpoint {
        ArtistReleasesEndpoint::new(self.client, self.id)
    }
}


impl<'a> InternalEndpoint<Artist> for ArtistEndpoint<'a> {}


impl<'a> Query for ArtistEndpoint<'a> {
    fn query(&self) -> String {
        format!("artists/{}", self.id)
    }
}

impl<'a> Endpoint<Artist> for ArtistEndpoint<'a> {
    fn get(&self) -> DiscographyResult<Artist> {
        self.fetch(self.client)
    }
}


#[cfg(test)]
mod tests {
    use discography::Discography;
    use Query;

    #[test]
    fn single_artist() {
        let discogs = Discography::new();

        let expected = "artists/1";

        let query = discogs.database().artists().id(1).query();

        assert_eq!(query, expected)
    }
}