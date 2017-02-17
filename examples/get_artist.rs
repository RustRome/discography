extern crate discography;


use discography::{Discography, Endpoint};

fn main() {
    let discogs = Discography::new();

    let artist = discogs.database()
        .artists().id(3840).get().expect("Artist not found");

    println!("{:?}", artist.name);
}