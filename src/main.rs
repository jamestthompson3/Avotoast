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

    let action: u8 = action.trim.parse()
        .expect("Not a valid Number");
}

enum ActionType {
    1,
    2,
    3
}

mod interpret {
    fn actions(action: ActionType) {

    }
}
