extern crate openssl;
extern crate pem;

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
    pub fn generate_key() {
        let rsa = Rsa::generate(4096).unwrap();
        let public_key = rsa.public_key_to_der().unwrap();
        let private_key = rsa.private_key_to_der().unwrap();
        let private_pem = Pem {
            tag: String::from("RSA PRIVATE KEY"),
            contents: private_key,
        };
        let private = encode(&private_pem);
        let mut file = File::create("avoWallet").write_all(private);

    }
    // Create public key -> stored on server
    // Save creditials --> server ---> decrpyts w/ public key ---->
    pub fn send_slice() {
        println!("slice sent")
    }
    pub fn check_transactions() {
        println!("transaction sent")
    }
}
