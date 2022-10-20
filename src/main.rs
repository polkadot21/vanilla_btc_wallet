use std::io;

const MAX_NUMBER_OF_ATTEMPTS: u8 = 3;

struct User{
    username: String,
    password: String
}


fn login() -> bool {

    let known_user = User {
        username: String::from("polkadot21"),
        password: String::from("123")
    };

    println!("Please enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read username");
    println!("Entered {}", username);

    if username.trim() != known_user.username {
        return false;
    }

    println!("Please enter your password:");
    let mut attempt = 1;

    while attempt <= MAX_NUMBER_OF_ATTEMPTS {

        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read username");
        println!("Password entered and hidden.");

        if password.trim() == known_user.password {
            return true;
        }
        else {
            let left_attempts = MAX_NUMBER_OF_ATTEMPTS - attempt;
            println!("Please, try again. You have {} attempts left", left_attempts);
            attempt = attempt + 1;
        }
    }
    println!("You have used all the attempts.");
    return false;
}


fn show_menu() {
    println! ("-----------------------------------------------");
    println! ("Choose one of the following options:");
    println! ("-----------------------------------------------");

    println! ("Press 1 to generate a random private key");
    println! ("Press 2 to generate a random Public key");
    println! ("Press 3 to generate a random Wallet Address");
    println! ("press 4 to create a multisig address");
    println! ("Press 5 to Check Balance");
    println! ("Press 6 Hash Address");
    println! ("Press 7 to Send Bitcoins");
    println! ("Press 8 to Push Transactions to Blockchain");
    println! ("Press 9 Send Bitcoin Info to Email");
    println! ("Press 10 to Send bitcoin Transtions to Email");
    println! ("Press 11 Send Notification to phone Number");
    println! ("Press Q to Exit");
    println! ("-----------------------------------------------");
}


fn main() {
    let logged_in = login();

    if logged_in {
        println!("You have successfully logged in!");
        show_menu();
    } else {
        println!("You were unable to login!")
    }
}
