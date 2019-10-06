use std::env;
use std::process;

use solidprofileimporter::facebook::ProfileInformation;
use solidprofileimporter::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem Passing Arguments: {}", err);
        process::exit(1)
    });

    let my_profile = ProfileInformation::new(&config.filename);
    println!("Got a name? {}", my_profile.unwrap().profile.name.full_name);
}
