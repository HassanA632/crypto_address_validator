use hex;
use sha3::{Digest, Keccak256};

pub fn length_prefix_check(address: &str) -> Result<(), &str> {
    //Length Check

    //if doesnt follow length/prefix standard
    if address.chars().count() != 42 || !address.starts_with("0x") {
        return Err("Invalid Length");
    };
    Ok(())
}

/*
EIP-55 checksum validation
Ethereum uses mix of uppercase and lowercase as checksum within 40 hex characters (exclude 0x)
Checksum is optional, so doesnt apply to all eth addresses.
*/

//Some sort of EIP-55 check?
pub fn does_address_contain_upper_and_lowercase(address: &str) -> Option<String> {
    let address: String = address.chars().skip(2).collect();
    let mut contains_upper = false;
    let mut contains_lower = false;
    //search for first instance of upper/lower, break once fulfilled
    for c in address.chars() {
        if c.is_ascii_uppercase() {
            contains_upper = true;
        } else if c.is_ascii_lowercase() {
            contains_lower = true;
        }
        if contains_lower == true && contains_upper == true {
            return Some(address.to_string());
        }
    }
    None
}

pub fn conduct_checksum(address: &str) {
    //.to_lowercase();
    //Create hasher state
    let mut hasher = Keccak256::new();
    //input address
    hasher.update(address.as_bytes());
    //finalise hash state
    let final_hash = hasher.finalize();
    //turn bytes into hexa-decimal string
    let final_hash = hex::encode(final_hash);
    println!("{:?}", final_hash);
}
#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    //Check length
    fn eth_length_prefix_checker_test() {
        let valid_address = "0x1111111122222222223333333333444444444455";
        let invalid_length_address = "0x11111111222222222233333333334444444444556";
        let invalid_prefix_address = "0f1111111122222222223333333333444444444455";
        assert_eq!(length_prefix_check(valid_address), Ok(()));
        assert_ne!(length_prefix_check(invalid_length_address), Ok(()));
        assert_ne!(length_prefix_check(invalid_prefix_address), Ok(()));
    }
    #[test]
    fn validate_upper_lower_func_test() {
        let address = "aBcDe";
        assert_eq!(
            does_address_contain_upper_and_lowercase(address),
            Some(address.to_string())
        );
        assert_ne!(
            does_address_contain_upper_and_lowercase(address),
            Some(address.to_ascii_lowercase())
        );
    }

    #[test]
    fn check_hashing() {
        let address: &str = "6eA66E412F4C9949d8cC2cf99112b6CC43A8Bfd6";
        conduct_checksum(address);
    }
}
