extern crate discography;


use discography::{Discography, Endpoint};

fn main() {
    let discogs = Discography::new();

    let label_releases = discogs.database()
        .labels().id(1).releases().get();

    println!("{:?}", label_releases);
}