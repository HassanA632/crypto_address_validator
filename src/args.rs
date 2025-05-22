use clap::{command, Arg, Command};

//build CLI before getting arg input
pub fn build_cli() -> Command {
    command!()
        //info regarding program, displayed using --help
        .about("This application takes a list of crypto addresses as input and returns a .txt file of valid crypto addresses based on certain criteria")
        .arg(
            //identifier for arg
            Arg::new("txtfile")
                .help("Filename containing address list to validate")
                //arg is required for program to function
                .required(true),
        )
}

//collect arg input
pub fn handle_args() -> String {
    build_cli()
        //collect arg's
        .get_matches()
        //use id to collect "txtfile"
        .get_one::<String>("txtfile")
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_valid_file_argument() {
        let get_matches = build_cli()
            //simulate arg entry
            .try_get_matches_from(["crypto_address_validator", "test.txt"])
            .unwrap()
            .get_one::<String>("txtfile")
            .unwrap()
            .to_string();
        assert_eq!(get_matches, "test.txt");
    }
}
