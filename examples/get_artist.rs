extern crate discography;


use discography::{Discography, EndPoint};

fn main() {
    let discogs = Discography::new();

    let label = discogs.database()
        .labels().id(1).get().expect("Label not found");

    println!("{:?}", label);
}