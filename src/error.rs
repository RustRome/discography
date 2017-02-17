use std::io;
use hyper;
use serde_json;


#[derive(Debug)]
pub struct ApiError {
    url: String,
    message: String,
}


impl ApiError {
    pub fn new(url: String, message: String) -> ApiError {
        ApiError {
            url: url,
            message: message,
        }
    }
}

#[derive(Debug)]
pub enum DiscographyError {
    Json(serde_json::Error),
    Http(hyper::Error),
    Api(ApiError),
    Io(io::Error),
}


impl From<io::Error> for DiscographyError {
    fn from(err: io::Error) -> DiscographyError {
        DiscographyError::Io(err)
    }
}

impl From<hyper::Error> for DiscographyError {
    fn from(err: hyper::Error) -> DiscographyError {
        DiscographyError::Http(err)
    }
}

impl From<serde_json::Error> for DiscographyError {
    fn from(err: serde_json::Error) -> DiscographyError {
        DiscographyError::Json(err)
    }
}
