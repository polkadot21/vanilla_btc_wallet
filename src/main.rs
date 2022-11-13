extern crate core;

mod btc_menu;

use std::io;
use std::ops::Add;
use std::str::FromStr;

use serde_json::{Result, Value};
use std::fs::File;

use secp256k1::{Secp256k1, Message};
use secp256k1::{ PublicKey, SecretKey};
use secp256k1::rand::thread_rng;
use bitcoin::{Address, Network, PrivateKey};
use bitcoin::util::address::Payload;



const MAX_NUMBER_OF_ATTEMPTS: u8 = 3;
const SEED: u64 = 1029120390912309201;

struct User{
    username: String,
    password: String
}

impl User {
    pub fn known_user() -> User {
        User {
            username: String::from("polkadot21"),
            password: String::from("123")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Menu {
    GenerateKeyPair,
    GenerateWalletAddress,
    CheckBalance,
    SendBTC,
    Quit,
    NotKnown,
}
#[derive(Debug)]
pub struct Choice {
    choice: Menu
}

impl Choice {
    pub fn new() -> Choice {
        let mut chosen_number = String::new();
        io::stdin().read_line(&mut chosen_number).expect("This option is not supported");
        return if chosen_number.trim() == "1" {
            Choice {
                choice: Menu::GenerateKeyPair
            }
        } else if chosen_number.trim() == "2" {
            Choice {
                choice: Menu::GenerateWalletAddress
            }
        } else if chosen_number.trim() == "3" {
            Choice {
                choice: Menu::CheckBalance
            }
        } else if chosen_number.trim() == "4" {
            Choice {
                choice: Menu::SendBTC
            }
        } else if chosen_number.trim() == "Q" {
            Choice {
                choice: Menu::Quit
            }
        } else {
            Choice {
                choice: Menu::NotKnown
            }
        }
    }

    pub fn choice(&self) -> Menu {
        self.choice
    }
}


fn login() -> bool {

    let known_user = User::known_user();

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


fn generate_key_pair() -> PublicKey {
    let secp = Secp256k1::new();

    let (secret_key, public_key) = secp.generate_keypair(&mut thread_rng());

    btc_menu::show_private_key_menu();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to understand the command!");
    if choice.trim() == "1" {
        save_private_key(&secret_key.to_string()).expect("TODO: panic message");

    }
    return public_key;
}


fn save_private_key(secret_key: &String) -> io::Result<()> {

    std::fs::write(
        "private_key.json",
        serde_json::to_string_pretty(&secret_key).unwrap())
}

// fn generate_wallet_address(public_key: Option<PublicKey>)  {
//
//     if public_key {
//
//     }
//
//     else if is_private_key_available() {
//         let private_key: PrivateKey = load_private_key();
//         let public_key = PublicKey::from_secret_key(&private_key, &Secp256k1);
//     }
//
// }

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
            let public_key = generate_key_pair();
            println!("public key: {}", &public_key.to_string());

        },

        Menu::GenerateWalletAddress => {
            let mut public_key = String::new();
            io::stdin().read_line(&mut public_key).expect("Failed to read public key");

            println!{"A new BTC wallet is being generated!"};
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
        let choice = Choice::new();
        let chosen_value: Menu = choice.choice;

        execute_command(chosen_value);

    } else {
        println!("You were unable to login!")
    }
}
