//!  A wrapper library for Discogs API
//!
//!
//! # Examples
//! ```rust,no_run
//! use discography::{Discography,Endpoint};
//!
//! let discogs = Discography::new();
//!
//! // RadioHead ID
//! let artist  = discogs.database().artists().id(3840).get()
//! .expect("Failed to fetch artist with id");
//! assert_eq!("Radiohead",artist.name);
//!
//! ```
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;


pub mod discography;
pub mod database;
pub mod models;
pub mod error;

use serde::Deserialize;
use error::DiscographyError;
pub use discography::Discography;

pub type DiscographyResult<T> = Result<T, DiscographyError>;

trait Query {
    fn query(&self) -> String;


    fn add_pagination<I>(&self, url: &mut String, page: I, per_page: I) where I: Into<Option<i16>> {

        per_page.into().map(|per_page| url.push_str(&format!("&per_page={}", per_page)));
        page.into().map(|page| url.push_str(&format!("&page={}", page)));

    }

    fn add_param(&self,params: & mut String , param: &str, value: &Option<String>) {
        if let &Some(ref val) = value {
            params.push_str(&format!("{}={}&",param,val));
        }
    }
}



trait IntoParam {
    fn to_param(&self) -> String;
}
trait InternalEndpoint<T: Deserialize>: Query {
    fn fetch(&self, dg: &Discography) -> DiscographyResult<T>
    {
        dg.get(self.query())
    }
}

pub trait Endpoint<T> {
    fn get(&self) -> DiscographyResult<T>;
}
