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

    interpret::actions(action)

}

mod interpret {
    pub fn actions(action: u8) {
        match action {
            1 => {println!("action is one");}
            2 => {println!("action is two");}
            3 => {println!("action is three");}
            _ => {println!("action is not covered");}
        };

    }
}
