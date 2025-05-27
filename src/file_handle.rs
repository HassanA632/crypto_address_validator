use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};

use crate::args;
use crate::validation_pipeline;

//Read input from user via CLI
fn read_input() -> String {
    //get first argument from CLI
    args::handle_args()
}

//convert a hashset to file: "filtered_btc.txt"
fn hashset_to_file(hashset_of_addresses: HashSet<&str>) -> io::Result<()> {
    let mut file = fs::File::create("filtered_btc.txt")?;
    for address in hashset_of_addresses {
        writeln!(file, "{}", address)?;
    }
    Ok(())
}

pub fn file_handler() {
    //Hashset of valid addresses
    let mut hashset_of_addresses = HashSet::new();

    let file_name = read_input();

    //read entire content of file into string
    let contents_of_file = fs::read_to_string(&file_name).expect("Failed to read file<<");

    //Iterate through addresses, each address is "current_address"
    for current_address in contents_of_file.lines() {
        println!("-------------------------------");
        println!("Checking: {}", &current_address);
        //put current address through validation pipeline
        match validation_pipeline::main_pipeline(current_address) {
            Ok(_) => {
                //If valid address, insert into hashset
                hashset_of_addresses.insert(current_address);
            }
            Err(_) => println!("INVALID ADDRESS"),
        }
        println!("-------------------------------");
    }
    //Converts hashset to .txt file
    hashset_to_file(hashset_of_addresses).unwrap();
}
