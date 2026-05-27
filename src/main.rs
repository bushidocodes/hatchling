use clap::Parser;
use hatchling::convert_facebook_to_solid;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(
    version,
    about = "A tool to convert Facebook data to Linked Data",
    author = "Sean McBride"
)]
struct Args {
    /// Path to the Facebook profile_information.json file
    input: String,

    /// Path for the resulting Turtle file
    output: String,

    /// Path to an optional friends file (DYI export or browser-scraped JSON)
    #[arg(short, long)]
    friends: Option<String>,
}

fn main() {
    let args = Args::parse();

    let profile = fs::read_to_string(&args.input).unwrap_or_else(|err| {
        eprintln!("Error reading {}: {}", args.input, err);
        process::exit(1);
    });

    let friends = args.friends.as_deref().map(|path| {
        fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading {}: {}", path, err);
            process::exit(1);
        })
    });

    let ttl = convert_facebook_to_solid(&profile, friends.as_deref()).unwrap_or_else(|err| {
        eprintln!("Conversion error: {}", err);
        process::exit(1);
    });

    fs::write(&args.output, ttl).unwrap_or_else(|err| {
        eprintln!("Error writing {}: {}", args.output, err);
        process::exit(1);
    });
}
