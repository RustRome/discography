extern crate discography;


use discography::{Discography, EndPoint};

fn main() {
    let discogs = Discography::new();

    let artist = discogs.database()
        .artists().id(1).get().expect("Artist not found");

    println!("{:?}", artist);
}