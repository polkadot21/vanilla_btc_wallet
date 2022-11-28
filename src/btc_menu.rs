

pub fn show_menu() {
    println! ("-----------------------------------------------");
    println!("You have successfully logged in!");
    println! ("Choose one of the following options:");
    println! ("-----------------------------------------------");

    println! ("Press 1 to generate a key pair");
    println! ("Press 2 to generate a random Wallet Address");
    println! ("Press 3 to Check Balance");
    println! ("Press 4 to Send Bitcoins");
    println! ("Press Q to Exit");

    println! ("-----------------------------------------------");
}


pub fn show_private_key_menu() {
    println! ("-----------------------------------------------");
    println!("Do you want to save your private key?");
    println! ("-----------------------------------------------");

    println! ("Press 1 to save the private key");
    println! ("Press 2 to forget the private key");

    println! ("-----------------------------------------------");

}