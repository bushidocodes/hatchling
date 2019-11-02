pub struct Config {
    pub facebook_zip: String,
    pub friends_json: String,
    pub output_file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Need a path to a Facebook zip! and a URL to a SOLID profile card");
        }
        let output_file = args[1].clone();
        let facebook_zip = args[2].clone();
        let friends_json = args[3].clone();

        Ok(Config {
            output_file,
            facebook_zip,
            friends_json,
        })
    }
}
