extern crate crypto;

use crypto::sha2::Sha256;
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

mod interpret {
    pub fn generate_key() {
        println!("key generated")
    }
    pub fn send_slice() {
        println!("slice sent")
    }
    pub fn check_transactions() {
        println!("transaction sent")
    }
}
