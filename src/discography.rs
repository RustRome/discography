use hyper;
use hyper::Client;
use hyper::header::UserAgent;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use database::Database;
use DiscographyResult;
use error::{DiscographyError, ApiError};
use serde::Deserialize;
use serde_json;
use std::io::Read;

///
pub struct Discography {
    url: String,
    user_agent: String,
    client: Client,
    token: Option<String>,
}

impl Discography {
    pub fn new() -> Discography {
        Discography::d_with_token(None)
    }

    fn d_with_token(token: Option<String>) -> Discography {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        Discography {
            url: String::from("https://api.discogs.com"),
            user_agent: String::from("Discography/1.0"),
            client: client,
            token: token,
        }
    }

    pub fn with_token<S>(token: S) -> Discography
        where S: Into<String>
    {
        Discography::d_with_token(Some(token.into()))
    }

    /// Database Endpoint https://www.discogs.com/developers/#page:database
    pub fn database(&self) -> Database {
        Database::new(self)
    }


    pub fn get<O>(&self, query: String) -> DiscographyResult<O>
        where O: Deserialize
    {
        let mut url = format!("{}/{}", self.url, query);

        self.token.as_ref().map(|t| url.push_str(&format!("&token={}", t)));

        let mut res = self.client
            .get(&url)
            .header(UserAgent(self.user_agent.clone()))
            .send()?;

        let mut full_resp = String::new();

        res.read_to_string(&mut full_resp)?;

        match res.status {
            hyper::status::StatusCode::Ok => {
                let data = serde_json::from_str(&full_resp)?;

                Ok(data)
            }
            _ => Err(DiscographyError::Api(ApiError::new(String::from(res.url.as_str()), full_resp))),
        }
    }
}
