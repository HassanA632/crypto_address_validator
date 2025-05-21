use bech32::decode;
use bs58;
use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq, Eq)]
pub enum AddressType {
    Base58,
    Bech32,
}

pub fn length_check(address: &str) -> Result<(), &str> {
    //Length Check
    let len_of_address = address.chars().count();

    if len_of_address < 26 || len_of_address > 62 {
        return Err("Invalid Length");
    };
    Ok(())
}

pub fn validate_base58(address: &str) -> Result<(), &str> {
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

pub fn validate_bech32(address: &str) -> Result<(), &str> {
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

pub fn prefix_check(address: &str) -> Result<AddressType, &str> {
    if address.starts_with("1") || address.starts_with("3") {
        return Ok(AddressType::Base58);
    } else if address.starts_with("bc1") {
        return Ok(AddressType::Bech32);
    } else {
        return Err("Prefix Error");
    };
}

pub fn match_address_type(address_type: AddressType, address: &str) -> Result<(), &str> {
    match address_type {
        AddressType::Base58 => validate_base58(address),
        AddressType::Bech32 => validate_bech32(address),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn length_checker_works() {
        //Length of 30
        let address = "111101111011110111101111011110";
        let length_check = length_check(address);
        assert_eq!(length_check, Ok(()));
    }

    #[test]
    fn prefix_checker_works() {
        //String starts with 3, so will return OK
        let address = "30001";
        let prefix_check = prefix_check(address);
        assert_eq!(prefix_check, Ok(AddressType::Base58));
    }

    #[test]
    fn base58_validation_check() {
        //Valid base58 address
        let address = "3JE2jZUsuFdyVq5PrUeyurhHXXZreLnYGE";
        let base58_check = validate_base58(address);
        assert_eq!(base58_check, Ok(()));
    }

    #[test]
    fn bech32_validation_check() {
        //Valid bech32 address
        let address = "bc1q82zkkc98azx7g3f8fyc8a2v9hz7wd609ca7t44";
        let bech32_check = validate_bech32(address);
        assert_eq!(bech32_check, Ok(()));
    }

    #[test]
    fn match_address_type_test() {
        let test_match =
            match_address_type(AddressType::Base58, "3JE2jZUsuFdyVq5PrUeyurhHXXZreLnYGE");
        assert_eq!(test_match, Ok(()))
    }
}
