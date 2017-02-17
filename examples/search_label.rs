extern crate discography;


use discography::{Discography, Endpoint};

fn main() {
    let discogs = Discography::new();

    let label = discogs.database()
        .search()
        .label("abbey")
        .get()
        .expect("Failed to search label");


    println!("{:?}", label);
}