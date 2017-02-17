use ::discography::Discography;
use {Query, InternalEndpoint, Endpoint, DiscographyResult};
use models::search::SearchResult;
use models::misc::Type;
use IntoParam;
pub struct SearchEndpoint<'a> {
    client: &'a Discography,
    query: Option<String>,
    q_type: Option<Type>,
    title: Option<String>,
    release_title: Option<String>,
    artist: Option<String>,
    label: Option<String>,
    genre: Option<String>,
    style: Option<String>,
    country: Option<String>,
    year: Option<i32>,
    page: Option<i16>,
    per_page: Option<i16>,
}


impl<'a> SearchEndpoint<'a> {
    pub fn new(client: &Discography) -> SearchEndpoint {
        SearchEndpoint {
            page: None,
            per_page: None,
            q_type: None,
            title: None,
            release_title: None,
            artist: None,
            label: None,
            genre: None,
            style: None,
            country: None,
            year: None,
            client: client,
            query: None
        }
    }

    pub fn q<T>(&'a mut self, query: T) -> &'a mut SearchEndpoint where T: Into<String> {
        self.query = Some(query.into());
        self
    }
    pub fn title<T>(&'a mut self, title: T) -> &'a mut SearchEndpoint where T: Into<String> {
        self.title = Some(title.into());
        self
    }
    pub fn release_title<T>(&'a mut self, release_title: T) -> &'a mut SearchEndpoint where T: Into<String> {
        self.release_title = Some(release_title.into());
        self
    }
    pub fn artist<T>(&'a mut self, artist: T) -> &'a mut SearchEndpoint where T: Into<String> {
        self.artist = Some(artist.into());
        self
    }
    pub fn label<T>(&'a mut self, label: T) -> &'a mut SearchEndpoint where T: Into<String> {
        self.label = Some(label.into());
        self
    }
    pub fn genre<T>(&'a mut self, genre: T) -> &'a mut SearchEndpoint where T: Into<String> {
        self.genre = Some(genre.into());
        self
    }
    pub fn style<T>(&'a mut self, style: T) -> &'a mut SearchEndpoint where T: Into<String> {
        self.style = Some(style.into());
        self
    }
    pub fn country<T>(&'a mut self, country: T) -> &'a mut SearchEndpoint where T: Into<String> {
        self.country = Some(country.into());
        self
    }
    pub fn year(&'a mut self, year: i32) -> &'a mut SearchEndpoint {
        self.year = Some(year);
        self
    }
    pub fn q_type(&'a mut self, q_type: Type) -> &'a mut SearchEndpoint {
        self.q_type = Some(q_type);
        self
    }

    pub fn pagination(&mut self, page: i16, per_page: i16) -> &'a mut SearchEndpoint {
        self.page = Some(page);
        self.per_page = Some(per_page);
        self
    }
}

impl<'a> InternalEndpoint<SearchResult> for SearchEndpoint<'a> {}


impl<'a> Query for SearchEndpoint<'a> {
    fn query(&self) -> String {
        let mut base = String::from("database/search?");


        self.add_param(&mut base, "q", &self.query);
        self.add_param(&mut base, "title", &self.title);


        self.add_param(&mut base, "release_title", &self.release_title);
        self.add_param(&mut base, "artist", &self.artist);
        self.add_param(&mut base, "label", &self.label);
        self.add_param(&mut base, "genre", &self.genre);
        self.add_param(&mut base, "style", &self.style);
        self.add_param(&mut base, "country", &self.country);

        // year
        self.add_param(&mut base, "year", &self.year.map(|y| {
            y.to_string()
        }));

        // type
        self.add_param(&mut base, "type", &self.q_type.as_ref().and_then(|v| {

            Some(v.to_param())

        }));


        let len = base.len() - 1;
        base.truncate(len);

        self.add_pagination(&mut base, self.page, self.per_page);

        base
    }
}


impl<'a> Endpoint<SearchResult> for SearchEndpoint<'a> {
    fn get(&self) -> DiscographyResult<SearchResult> {
        self.fetch(self.client)
    }
}


#[cfg(test)]
mod tests {
    use discography::Discography;
    use Query;
    use models::misc::Type;

    #[test]
    fn search_database_with_query() {
        let discogs = Discography::new();

        let expected = "database/search?q=nirvana";

        let query = discogs.database().search().q("nirvana").query();

        assert_eq!(query, expected)
    }

    #[test]
    fn search_database_with_release_title_and_artist() {
        let discogs = Discography::new();

        let expected = "database/search?release_title=nevermind&artist=nirvana&per_page=3&page=1";

        let query = discogs.database()
            .search()
            .release_title("nevermind")
            .artist("nirvana")
            .pagination(1, 3)
            .query();

        assert_eq!(query, expected)
    }

    #[test]
    fn search_database_with_query_and_type() {
        let discogs = Discography::new();

        let expected = "database/search?q=nirvana&type=artist&per_page=3&page=1";

        let query = discogs.database()
            .search()
            .q("nirvana")
            .q_type(Type::Artist)
            .pagination(1, 3)
            .query();

        assert_eq!(query, expected)
    }
}
