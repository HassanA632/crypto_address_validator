use std::collections::HashSet;
use std::fs;
use std::io;

use crate::validation_pipeline;

pub fn file_handler() {
    //Hashset of valid addresses
    let mut hashset_of_addresses = HashSet::new();

    let file_name = read_input();

    //read entire content of file into string
    let contents_of_file = fs::read_to_string(&file_name).expect("Failed to read file");

    //Iterate and check each address
    for current_address in contents_of_file.lines() {
        println!("-------------------------------");
        println!("Checking: {}", &current_address);
        match validation_pipeline::process(current_address) {
            Ok(_) => {
                hashset_of_addresses.insert(current_address);
            }
            Err(_) => println!("INVALID ADDRESS"),
        }
        println!("-------------------------------");
    }

    //Todo- Make this a seperate txt file
    println!("Hashset of valid address:");
    println!("{:?}", hashset_of_addresses);
}

//Read input from user.
fn read_input() -> String {
    //Placeholder for .txt file to check
    let mut file_name: String = String::new();

    //Read file name
    println!("Enter file name: ");
    io::stdin()
        .read_line(&mut file_name)
        .expect("Error reading file name");

    //Trim whitespace
    file_name.trim().to_string()
}
