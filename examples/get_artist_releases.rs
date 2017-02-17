extern crate discography;


use discography::{Discography, Endpoint};

fn main() {
    let discogs = Discography::new();

    let releases = discogs.database()
        .artists().id(1).releases().get().expect("Artist not found");

    println!("{:?}", releases);
}