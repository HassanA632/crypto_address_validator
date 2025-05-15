use crate::bitcoin_check::*;
pub fn process(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    length_check(address)?;
    println!("✅ Length Check");
    prefix_check(address)?;
    println!("✅ Prefix Check");
    let which_prefix = prefix_check(address)?;
    match_address_type(which_prefix, address)?;
    Ok(())
}
