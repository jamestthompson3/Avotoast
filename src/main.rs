extern crate openssl;
extern crate pem;
#[macro_use]
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate byteorder;
extern crate base64;


use std::io;
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
    use openssl::sign::{Signer};
    use openssl::pkey::PKey;
    use openssl::hash::MessageDigest;
    use std::time::{SystemTime, UNIX_EPOCH};

    use pem:: {Pem, encode};
    use std::fs::File;
    use std::io::prelude::*;

    use base64;

    use std::io::{self, Write};
    use futures::{Future, Stream};
    use hyper::Client;
    use tokio_core::reactor::Core;
    use hyper::{Method, Request};
    use hyper::header::{ContentLength, ContentType};
    use byteorder::{BigEndian, WriteBytesExt};

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
            Err(why) => panic!("couldn't create, {}", why),
            Ok(file) => file,
        };
        match file.write_all(private.as_bytes()) {
            Err(why) => {
                panic!("Couldn't write, {}", why)
            },
            Ok(_) => println!("Key generated"),
        };
        println!("Your Wallet Address is\n{}", public);
    }
    // Create public key -> stored on server
    // Save creditials --> server ---> decrpyts w/ public key ---->
    pub fn send_slice() {
        println!("Enter your PUBLIC wallet key");
        let mut from_address = String::new();
        io::stdin().read_line(&mut from_address)
            .expect("failed to read from wallet");


        println!("Enter the wallet you want to send money to");
        let mut to_address = String::new();
        io::stdin().read_line(&mut to_address)
            .expect("failed to read to wallet");

        println!("How much money do you want to send?");
        let mut amount_input = String::new();
        io::stdin().read_line(&mut amount_input)
            .expect("failed to read amount");
        let amount: u32 = amount_input.trim().parse()
                .expect("Not a valid Number");

        let mut core = Core::new()
            .expect("Couldn't instantiate");
        let client = Client::new(&core.handle());


        let signature = create_message();

        let json = json!({
                "from": from_address,
                "to": to_address,
                "amount": amount,
                "message": signature
            });

        let uri = "http://192.168.1.85:8080/".parse()
            .expect("couldn't parse uri");
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.to_string().len() as u64));
        req.set_body(json.to_string());

        let post = client.request(req).and_then(|res| {
            println!("POST: {}", res.status());

            res.body().concat2()
        });

        core.run(post).expect("Could not post message");

        println!("slice sent")
    }

    pub fn create_message() -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

        let mut current_time = vec![];
        current_time.write_u16::<BigEndian>(now.as_secs() as u16).unwrap();

        let mut file = File::open("AvoWalletPrivateKey").expect("Cannot open private key");
        let mut contents = vec![];
        file.read_to_end(&mut contents).expect("Fail to read private key");
        // let private_key = parse(contents).unwrap().contents;
        let private_rsa = PKey::private_key_from_pem(&contents).unwrap();

        let mut signer = Signer::new(MessageDigest::sha256(), &private_rsa).unwrap();
        signer.update(&current_time).unwrap();
        let signature = signer.sign_to_vec().unwrap();
        return base64::encode(&signature);
    }

    pub fn check_transactions() {
        println!("transaction sent")
    }
}
