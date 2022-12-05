extern crate core;

mod btc_menu;

use std::fmt::format;
use std::io;
use std::ops::Add;
use std::str::FromStr;
use std::str;

use serde_json::{Result, Value};
use std::fs::File;

use secp256k1::{Secp256k1, Message};
use secp256k1::{ PublicKey, SecretKey};
use secp256k1::rand::thread_rng;
use bitcoin::{Address, Network, PrivateKey};
use bitcoin::util::address::Payload;
use bs58;
use base16;
use hex;
use hex::FromHex;

use sha256::digest;
use ripemd::{Ripemd160};
use whirlpool::{Whirlpool};
use sha2::{Digest, Sha256};


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
    public_key
}


fn save_private_key(secret_key: &String) -> io::Result<()> {

    std::fs::write(
        "private_key.json",
        serde_json::to_string_pretty(&secret_key).unwrap())
}

fn get_pbk_with_version(public_key: String) -> String {
    let version = "00";

    //sha256
    let mut hasher_sha256 = Sha256::new();
    hasher_sha256.update(public_key);
    let hashed_sha256 = hasher_sha256.finalize();

    //ripemd160
    let mut hasher_ripemd160 = Ripemd160::new();
    hasher_ripemd160.update(hashed_sha256);
    let hashed_ripemd160 = hasher_ripemd160.finalize();

    let hashed_ripemd160_str = format!("{:X}", hashed_ripemd160).to_lowercase();
    let pbk_with_version_str = String::from(version)  + &hashed_ripemd160_str;

    pbk_with_version_str
}

fn get_checksum_4_bytes(pbk_with_version: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&pbk_with_version);
    let once_hashed = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(once_hashed);
    let checksum = &hasher.finalize();

    let checksum_str = format!("{:X}", checksum).to_lowercase();
    let check_sum_4_byte = &checksum_str[0..8];

    let check_sum_4_byte_str = String::from(check_sum_4_byte);

    check_sum_4_byte_str
}

fn get_pbk_with_version_and_checksum(pbk_with_version: String, checksum: String) -> String {
    pbk_with_version + &checksum
}


fn encode_pbk_with_version_and_checksum_to_base58(pbk_with_version_and_checksum: String) -> String {
    let decoded_pbk_with_version_and_checksum = hex::decode(&pbk_with_version_and_checksum).unwrap();
    let encoded_base58 = bs58::encode(decoded_pbk_with_version_and_checksum).into_string();

    encoded_base58
}

fn generate_wallet_address(public_key: String) -> String {

    let pbk_with_version_str = get_pbk_with_version(public_key);
    let checksum_4_bytes: String = get_checksum_4_bytes(&pbk_with_version_str);
    let pbk_with_version_and_checksum = get_pbk_with_version_and_checksum(pbk_with_version_str, checksum_4_bytes) ;
    let wallet: String = encode_pbk_with_version_and_checksum_to_base58(pbk_with_version_and_checksum);

    wallet

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
            let public_key = generate_key_pair();
            println!("public key: {}", &public_key.to_string());

        },

        Menu::GenerateWalletAddress => {
            println!("Please enter your public key:");
            let mut public_key = String::new();
            io::stdin().read_line(&mut public_key).expect("Failed to read public key");
            generate_wallet_address(public_key);

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


#[cfg(test)]
mod tests {
    use crate::{encode_pbk_with_version_and_checksum_to_base58, generate_wallet_address, get_checksum_4_bytes, get_pbk_with_version, get_pbk_with_version_and_checksum};

    #[test]
    fn test_pbk_with_version() {
        let public_key = String::from("035718e87a0e5220efa55f0d220c561057939c35875ba48a05d3e149ff795ed320");
        let known_pbk_with_version: String = String::from("005e44c38815bfea90b2f6d6949d0d06e20d59f71e");

        let pbk_with_version: String = get_pbk_with_version(public_key);
        assert_eq!(pbk_with_version, known_pbk_with_version);
    }

    #[test]
    fn test_checksum_4_bytes() {
        let known_pbk_with_version: String = String::from("005e44c38815bfea90b2f6d6949d0d06e20d59f71e");
        let known_checksum_4_bytes: String = String::from("317563fa");

        let checksum_4_bytes = get_checksum_4_bytes(&known_pbk_with_version);
        assert_eq!(checksum_4_bytes, known_checksum_4_bytes);
    }

    #[test]
    fn test_pbk_with_version_and_checksum() {
        let known_pbk_with_version: String = String::from("005e44c38815bfea90b2f6d6949d0d06e20d59f71e");
        let known_checksum: String = String::from("317563fa");
        let known_pbk_with_version_checksum: String = String::from("005e44c38815bfea90b2f6d6949d0d06e20d59f71e317563fa");

        let pbk_with_version_and_checksum = get_pbk_with_version_and_checksum(known_pbk_with_version, known_checksum);
        assert_eq!(pbk_with_version_and_checksum, known_pbk_with_version_checksum);
    }


    #[test]
    fn test_encode_to_base58(){
        let known_pbk_with_version_checksum: String = String::from("005e44c38815bfea90b2f6d6949d0d06e20d59f71e317563fa");
        let known_wallet: String = String::from("19bStpozPcjHraqE4YC4RvwCYNeKdvAxqf");

        let wallet: String = encode_pbk_with_version_and_checksum_to_base58(known_pbk_with_version_checksum);
        assert_eq!(wallet, known_wallet);


    }

    #[test]
    fn test_generate_wallet(){
        let public_key = String::from("035718e87a0e5220efa55f0d220c561057939c35875ba48a05d3e149ff795ed320");
        let known_wallet: String = String::from("19bStpozPcjHraqE4YC4RvwCYNeKdvAxqf");

        let wallet: String = generate_wallet_address(public_key);
        assert_eq!(wallet, known_wallet)
    }
}