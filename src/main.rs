use std::env;
use std::process;

use hatchling::argument_parser::Config;
use hatchling::convert_facebook_to_solid;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem Passing Arguments: {}", err);
        process::exit(1)
    });

    let solid_profile = convert_facebook_to_solid(&config.facebook_zip, &config.friends_json)
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1)
        });;

    let mut file = File::create(&config.output_file).unwrap();
    file.write_all(solid_profile.as_bytes()).unwrap();
}
