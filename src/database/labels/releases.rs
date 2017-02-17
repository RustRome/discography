use ::discography::Discography;
use {Query, InternalEndpoint, Endpoint, DiscographyResult};

use models::label::LabelReleases;

pub struct LabelReleasesEndpoint<'a> {
    id: i64,
    client: &'a Discography,
    page: i16,
    per_page: i16,
}

impl<'a> LabelReleasesEndpoint<'a> {
    pub fn new(client: &Discography, id: i64) -> LabelReleasesEndpoint {
        LabelReleasesEndpoint {
            id: id,
            client: client,
            page: 1,
            per_page: 50,
        }
    }


    pub fn pagination(&mut self, page: i16, per_page: i16) -> &'a mut LabelReleasesEndpoint {
        self.page = page;
        self.per_page = per_page;
        self
    }
}


impl<'a> InternalEndpoint<LabelReleases> for LabelReleasesEndpoint<'a> {}

impl<'a> Query for LabelReleasesEndpoint<'a> {
    fn query(&self) -> String {
        format!("labels/{}/releases", self.id)
    }
}


impl<'a> Endpoint<LabelReleases> for LabelReleasesEndpoint<'a> {
    fn get(&self) -> DiscographyResult<LabelReleases> {
        self.fetch(self.client)
    }
}


#[cfg(test)]
mod tests {
    use discography::Discography;
    use Query;

    #[test]
    fn single_label_releases() {
        let discogs = Discography::new();

        let expected = "labels/1/releases";

        let query = discogs.database().labels().id(1).releases().query();

        assert_eq!(query, expected)
    }
}
