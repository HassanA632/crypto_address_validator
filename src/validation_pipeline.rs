use crate::bitcoin_check::*;
use std::error::Error;

pub fn main_pipeline(address: &str) -> Result<(), Box<dyn Error>> {
    bitcoin_pipeline(address)
}

fn bitcoin_pipeline(address: &str) -> Result<(), Box<dyn Error>> {
    length_check(address)?;
    println!("✅ Length Check");
    prefix_check(address)?;
    println!("✅ Prefix Check");
    let which_prefix = prefix_check(address)?;
    match_address_type(which_prefix, address)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::validation_pipeline::bitcoin_pipeline;
    //use pretty_assertions::assert_eq;

    #[test]
    fn test_bitcoin_pipeline() {
        let valid_address = "bc1qcxljd2xfcxu7utwj57c32p9wf4ht57j2kj73zj";
        let invalid_address = "bc1qcxljd2xfcxu7utwj57c32p9wf4ht57j2kj730j";
        assert!(bitcoin_pipeline(valid_address).is_ok());
        assert!(bitcoin_pipeline(invalid_address).is_err());
    }
    #[test]
    fn test_ethereum_pipieline() {
        let valid_address = "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97";
        let invalid_address = "0x4838B106FCeO647Bdf1E7877BF73cE8B0BAD5f97";
        //assert!(ethereum_pipeline(valid_address).is_ok());
        //assert!(ethereum_pipeline(invalid_address).is_err());
    }
}
