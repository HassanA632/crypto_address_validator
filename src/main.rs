use bech32::decode;
use bs58;
use sha2::{Digest, Sha256};
use std::{collections, io::stdin, ops::Add};

#[derive(Debug)]
enum AddressType {
    Base58Check,
    Bech32,
    Bech32m,
    Empty,
}

fn length_check(address: &str) -> Option<bool> {
    //Length Check
    let len_of_address = address.chars().count();

    if len_of_address < 26 || len_of_address > 62 {
        return Some(false);
    };
    Some(true)
}

fn prefix_check(address: &str) -> Option<AddressType> {
    if address.starts_with("1") {
        return Some(AddressType::Base58Check);
    } else if address.starts_with("3") {
        return Some(AddressType::Base58Check);
    } else if address.starts_with("bc1") {
        return Some(AddressType::Bech32);
    } else if address.starts_with("bc1p") {
        return Some(AddressType::Bech32m);
    } else {
        return None;
    };
}

fn is_valid_base58check(address: &str) -> () {
    let decoded = match bs58::decode(address).into_vec() {
        Ok(bytes) => {
            //Check data is 25 bytes (Bitcoin Standard)
            if bytes.len() != 25 {
                None
            } else {
                //Split data between data and checksum
                let (data, checksum) = bytes.split_at(21);
                let hash = Sha256::digest(&Sha256::digest(data));
                if checksum == &hash[0..4] {
                    Some(bytes)
                } else {
                    None
                }
            }
        }
        Err(_) => None,
    };

    match decoded {
        Some(bytes) => println!("{:?}", bytes),
        None => println!("none"),
    }
}

fn validate_bech32_address(address: &str) -> bool {
    match decode(address) {
        Ok((_hrp, _data)) => true,
        Err(_) => false,
    }
}

fn main() {
    // //let mut input = String::new();
    // println!("Enter a crypto address to verify");
    // stdin().read_line(&mut input).expect("Failed to read line");
    // let trimmed_address = input.trim();
    let trimmed_address = "A1LQFM3A";
    //println!("{:?}", length_check(trimmed_address))
    println!("{}", validate_bech32_address(trimmed_address))
}
