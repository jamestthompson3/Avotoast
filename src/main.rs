extern crate openssl;
extern crate pem;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io;
use std::fs::File;


fn main() {
    println!("******************************************************\n");
    println!("Welcome to Avotoast ! The cryptocurrency of your dreams");
    println!("\n*****************************************************");
    println!("\nLet's get started!\n");
    println!("Select an option:\n 1. Start a new plate\n 2. Send a slice to your friends\n 3. Check your plate");

    let mut action = String::new();
    io::stdin().read_line(&mut action)
        .expect("failed to read option");

    let action: u8 = action.trim().parse()
        .expect("Not a valid Number");

        match action {
            1 => {interpret::generate_key();}
            2 => {interpret::send_slice();}
            3 => {interpret::check_transactions();}
            _ => {println!("action is not valid");}
        };

}

// TODO

mod interpret {
    use openssl::rsa::Rsa;
    use pem:: {Pem, encode};
    use std::fs::File;
    use std::io::prelude::*;

    use std::io::{self, Write};
    use futures::{Future, Stream};
    use hyper::Client;
    use tokio_core::reactor::Core;
    use hyper::{Method, Request};
    use hyper::header::{ContentLength, ContentType};

    pub fn generate_key() {
        let rsa = Rsa::generate(4096).unwrap();
        let public_key = rsa.public_key_to_der().unwrap();
        let private_key = rsa.private_key_to_der().unwrap();
        let private_pem = Pem {
            tag: String::from("RSA PRIVATE KEY"),
            contents: private_key,
        };
        let private = encode(&private_pem);
        let public_pem = Pem {
            tag: String::from("RSA PUBLIC KEY"),
            contents: public_key,
        };
        let public = encode(&public_pem);
        let mut file = match File::create("AvoWalletPrivateKey") {
            Err(why) => panic!("couldn't create"),
            Ok(file) => file,
        };
        match file.write_all(private.as_bytes()) {
            Err(why) => {
                panic!("Couldn't write")
            },
            Ok(_) => println!("Key generated"),
        };
        println!("Your Wallet Address is\n{}", public);
    }
    // Create public key -> stored on server
    // Save creditials --> server ---> decrpyts w/ public key ---->
    pub fn send_slice() {
        let core = Core::new()
            .expect("Couldn't instantiate");
        let client = Client::new(&core.handle());

        let json = r#"{"library":"hyper"}"#;
        let uri = "http://127.0.0.1/post".parse()
            .expect("couldn't parse uri");
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = client.request(req).and_then(|res| {
            println!("POST: {}", res.status());

            res.body().concat2()
        });

        println!("slice sent")
    }
    pub fn check_transactions() {
        println!("transaction sent")
    }
}
