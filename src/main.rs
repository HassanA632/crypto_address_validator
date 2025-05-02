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

fn is_valid_base58check(address: &str) -> bool {
    let decoded = match bs58::decode(address).into_vec() {
        Ok(bytes) => true,
        Err(_) => return false,
    };
    decoded
}

fn main() {
    // //let mut input = String::new();
    // println!("Enter a crypto address to verify");
    // stdin().read_line(&mut input).expect("Failed to read line");
    // let trimmed_address = input.trim();
    let trimmed_address = "bc1pddddddddddddddddddddfddddddddddd";
    //println!("{:?}", length_check(trimmed_address))
    println!("{:?}", prefix_check(trimmed_address))
}
