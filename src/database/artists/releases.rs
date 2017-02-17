use ::discography::Discography;
use {Query, InternalEndpoint, Endpoint, DiscographyResult};
use models::artist::ArtistReleases;


pub struct ArtistReleasesEndpoint<'a> {
    client: &'a Discography,
    id: i64,
    page: i16,
    per_page: i16,
}


impl<'a> ArtistReleasesEndpoint<'a> {
    pub fn new(client: &Discography, id: i64) -> ArtistReleasesEndpoint {
        ArtistReleasesEndpoint {
            id: id,
            client: client,
            page: 1,
            per_page: 50,
        }
    }

    pub fn pagination(&mut self, page: i16, per_page: i16) -> &'a mut ArtistReleasesEndpoint {
        self.page = page;
        self.per_page = per_page;
        self
    }
}


impl<'a> InternalEndpoint<ArtistReleases> for ArtistReleasesEndpoint<'a> {}


impl<'a> Query for ArtistReleasesEndpoint<'a> {
    fn query(&self) -> String {
        format!("artists/{}/releases", self.id)
    }
}

impl<'a> Endpoint<ArtistReleases> for ArtistReleasesEndpoint<'a> {
    fn get(&self) -> DiscographyResult<ArtistReleases> {
        self.fetch(self.client)
    }
}


#[cfg(test)]
mod tests {
    use discography::Discography;
    use Query;

    #[test]
    fn single_artist_releases() {
        let discogs = Discography::new();

        let expected = "artists/1/releases";

        let query = discogs.database().artists().id(1).releases().query();

        assert_eq!(query, expected)
    }
}
