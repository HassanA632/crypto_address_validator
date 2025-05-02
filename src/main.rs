use bech32::decode;
use bs58;
use sha2::{Digest, Sha256};
use std::{collections, io::stdin, ops::Add};

#[derive(Debug)]
enum AddressType {
    Base58,
    Bech32,
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
    if address.starts_with("1") || address.starts_with("3") {
        return Some(AddressType::Base58);
    } else if address.starts_with("bc1") {
        return Some(AddressType::Bech32);
    } else {
        return None;
    };
}

fn validate_base58(address: &str) -> bool {
    let decoded = match bs58::decode(address).into_vec() {
        Ok(bytes) => {
            //Check data is 25 bytes (Bitcoin Standard)
            if bytes.len() != 25 {
                None
            } else {
                //Split data between data and checksum
                let (data, checksum) = bytes.split_at(21);
                let hash = Sha256::digest(&Sha256::digest(data));
                if checksum == hash.get(0..4).unwrap() {
                    Some(bytes)
                } else {
                    None
                }
            }
        }
        Err(_) => None,
    };

    match decoded {
        Some(_) => {
            println!("Valid Base58");
            true
        }
        None => {
            println!("NOT Valid Base58");
            false
        }
    }
}

fn validate_bech32(address: &str) -> bool {
    match decode(address) {
        Ok((_hrp, _data)) => {
            println!("Valid bech32");
            true
        }
        Err(_) => {
            println!("NOT Valid bech32");
            false
        }
    }
}

fn main() {
    // //let mut input = String::new();
    // println!("Enter a crypto address to verify");
    // stdin().read_line(&mut input).expect("Failed to read line");
    // let trimmed_address = input.trim();
    let trimmed_address = "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq";

    let address_type: Option<AddressType> = prefix_check(trimmed_address);
    //println!("{:#?}", address_type);

    let my_match: bool = match address_type {
        Some(my_type) => {
            println!("Successful prefix check");
            match my_type {
                AddressType::Base58 => validate_base58(trimmed_address),
                AddressType::Bech32 => validate_bech32(trimmed_address),
            }
        }
        None => {
            println!("unsuccessful prefix check");
            false
        }
    };
    println!("{}", my_match)
}
