use ::discography::Discography;
use {Query, InternalEndpoint, Endpoint, DiscographyResult};
use models::label::Label;
use database::labels::releases::LabelReleasesEndpoint;


pub struct LabelEndpoint<'a> {
    id: i64,
    client: &'a Discography,
}


impl<'a> LabelEndpoint<'a> {
    pub fn new(client: &Discography, id: i64) -> LabelEndpoint {
        LabelEndpoint {
            id: id,
            client: client,
        }
    }

    pub fn releases(&self) -> LabelReleasesEndpoint {
        LabelReleasesEndpoint::new(self.client, self.id)
    }
}


impl<'a> InternalEndpoint<Label> for LabelEndpoint<'a> {}

impl<'a> Query for LabelEndpoint<'a> {
    fn query(&self) -> String {
        format!("labels/{}", self.id)
    }
}


impl<'a> Endpoint<Label> for LabelEndpoint<'a> {
    fn get(&self) -> DiscographyResult<Label> {
        self.fetch(self.client)
    }
}


#[cfg(test)]
mod tests {
    use discography::Discography;
    use Query;

    #[test]
    fn single_label() {
        let discogs = Discography::new();

        let expected = "labels/10";

        let query = discogs.database().labels().id(10).query();

        assert_eq!(query, expected)
    }
}
