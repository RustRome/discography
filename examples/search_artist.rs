extern crate discography;
use std::io::{self,Write};
use discography::models::misc::Type;
use discography::{Discography, Endpoint};

fn main() {


    io::stdout().write(b"Insert a token: ").unwrap();

    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let discogs = Discography::with_token(buffer);

    let artist = discogs.database()
        .search()
        .artist("nirvana")
        .q_type(Type::Label)
        .pagination(1,5)
        .get()
        .expect("Failed to search artist");


    println!("{:?}", artist);
}