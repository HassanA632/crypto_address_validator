use bech32::decode;
use bs58;
use sha2::{Digest, Sha256};
//use std::{collections, io::stdin, ops::Add};

#[derive(Debug)]
enum AddressType {
    Base58,
    Bech32,
}

fn length_check(address: &str) -> Result<(), &str> {
    //Length Check
    let len_of_address = address.chars().count();

    if len_of_address < 26 || len_of_address > 62 {
        return Err("Invalid Length");
    };
    Ok(())
}

fn validate_base58(address: &str) -> Result<(), &str> {
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
        Some(_bytes) => {
            println!("✅ Valid Base58 Layout");
            Ok(())
        }
        None => {
            println!("❌ NOT Valid Base58 Layout");
            Err("Invalid Base58")
        }
    }
}

fn validate_bech32(address: &str) -> Result<(), &str> {
    match decode(address) {
        Ok((_hrp, _data)) => {
            println!("✅ Valid bech32 Layout");
            Ok(())
        }
        Err(_) => {
            println!("❌ NOT Valid bech32 Layout");
            Err("Invalid Bech32")
        }
    }
}

fn prefix_check(address: &str) -> Result<AddressType, &str> {
    if address.starts_with("1") || address.starts_with("3") {
        return Ok(AddressType::Base58);
    } else if address.starts_with("bc1") {
        return Ok(AddressType::Bech32);
    } else {
        return Err("Prefix Error");
    };
}

fn match_address_type(address_type: AddressType, address: &str) -> Result<(), &str> {
    match address_type {
        AddressType::Base58 => validate_base58(address),
        AddressType::Bech32 => validate_bech32(address),
    }
}

fn process(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    length_check(address)?;
    println!("✅ Length Check");
    prefix_check(address)?;
    println!("✅ Prefix Check");
    let which_prefix = prefix_check(address)?;
    match_address_type(which_prefix, address)?;
    Ok(())
}

fn main() {
    let trimmed_address = "3Jowo3eW88owPZScgFk";
    match process(trimmed_address) {
        Ok(_) => println!("Valid Bitcoin Address"),
        Err(_) => println!("Invalid Bitcoin Address"),
    }
}
