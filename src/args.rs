use clap::{command, Arg};
pub fn handle_args() -> String {
    let match_result = command!()
        .about("This application takes a list of crypto addresses as input and returns a .txt file of valid crypto addresses based on certain criteria")
        .arg(
            Arg::new("txtfile")
                .help("Filename containing address list to validate")
                .required(true),
        )
        .get_matches();
    match_result
        .get_one::<String>("txtfile")
        .unwrap()
        .to_string()
}
