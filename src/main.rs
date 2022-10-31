extern crate core;

mod btc_menu;

use std::io;
use std::ops::Add;
use std::str::FromStr;
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey
};
use bitcoin::{ Address, Network };
use bitcoin::util::address::Payload;


const MAX_NUMBER_OF_ATTEMPTS: u8 = 3;
const SEED: u64 = 42;

struct User{
    username: String,
    password: String
}

#[derive(Debug)]
enum Menu {
    GenerateKeyPair,
    GenerateWalletAddress,
    CheckBalance,
    SendBTC,
    Quit,
    NotKnown,
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


fn generate_key_pair() -> (SecretKey, PublicKey){
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(SEED);

    secp.generate_keypair(&mut rng)
}


fn generate_wallet_address(public_key: &String)  {

}

fn check_balance() {

}

fn hash_address() {

}

fn push_transactions_to_blockchain() {

}



fn execute_command(choice: Menu) {
    println!("{:?}", choice);
    match choice {
        Menu::GenerateKeyPair => {
            println!{"A key pair is being generated!"};
            let (secret_key, public_key) = generate_key_pair();
            println!("secret key: {}", &secret_key.to_string());
            println!("public key: {}", &public_key.to_string());

        },

        Menu::GenerateWalletAddress => {
            let mut public_key = String::new();
            io::stdin().read_line(&mut public_key).expect("Failed to read public key");

            println!{"A new BTC wallet is being generated!"};
            // let address = generate_wallet_address(&public_key);
            // println!("{}", address);
        },

        Menu::CheckBalance => {
            println!{"The balance is being checked!"};
        },

        Menu::SendBTC => {
            println!{"BTC is being sent!"};
        },

        Menu::Quit => {
            println!{"Quitting!"};
        },

        Menu::NotKnown => {
            println!("This command is not supported!");
            println!{"Quitting!"};
        },
    }
}


fn main() {
    let logged_in = login();

    if logged_in {
        btc_menu::show_menu();

        //TODO: change to an int or match whether int or &str
        let mut choice;
        let mut chosen_number = String::new();
        io::stdin().read_line(&mut chosen_number).expect("This option is not supported");

        if chosen_number.trim() == "1" {
            choice = Menu::GenerateKeyPair;
        } else if  chosen_number.trim() == "2" {
            choice = Menu::GenerateWalletAddress;
        } else if chosen_number.trim() == "3" {
            choice = Menu::CheckBalance;
        } else if chosen_number.trim() == "4" {
            choice = Menu::SendBTC;
        } else if chosen_number.trim() == "Q" {
            choice = Menu::Quit;
        } else {
            choice = Menu::NotKnown;
        }

        execute_command(choice);

    } else {
        println!("You were unable to login!")
    }
}
