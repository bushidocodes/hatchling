use std::env;
use std::process;

use solidprofileimporter::facebook::{FBProfileInformation, FBFriends};
use solidprofileimporter::Config;
use solidprofileimporter::solidprofile::Profile;

use rdf::writer::turtle_writer::TurtleWriter;
use rdf::writer::rdf_writer::RdfWriter;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem Passing Arguments: {}", err);
        process::exit(1)
    });

    let my_fb_profile = FBProfileInformation::new(&config.facebook_zip).unwrap_or_else(|err| {
        eprintln!("Problem Parsing FB zip: {}", err);
        process::exit(1)
    });

    let my_fb_friends = FBFriends::new(&config.friends_json).unwrap_or_else(|err| {
        eprintln!("Problem Parsing friends json: {}", err);
        process::exit(1)
    });

    let mut profile = Profile::new();

    // foaf name
    if !my_fb_profile.profile.name.full_name.is_empty() {
        profile.set_name(&my_fb_profile.profile.name.full_name);
    }

    // foaf familyName / lastName
    if !my_fb_profile.profile.name.last_name.is_empty() {
        profile.set_last_name(&my_fb_profile.profile.name.last_name);
    }

    // foaf givenName / firstName     
    if !my_fb_profile.profile.name.first_name.is_empty() {
        profile.set_first_name(&my_fb_profile.profile.name.first_name);
    }

    // foaf gender
    if !my_fb_profile.profile.gender.gender_option.is_empty() {
        profile.set_gender(&my_fb_profile.profile.gender.gender_option);
    }

    // foaf birthday and age - Calculations seems to not quite correct
    if !my_fb_profile.profile.birthday.month > 0 && !my_fb_profile.profile.birthday.day > 0 && !my_fb_profile.profile.birthday.year > 0{
        profile.set_birthday_and_age(
            my_fb_profile.profile.birthday.month.into(), 
            my_fb_profile.profile.birthday.day.into(), 
            my_fb_profile.profile.birthday.year.into()
        );
    }

    // foaf phone
    for elem in my_fb_profile.profile.phone_numbers.iter() {
        profile.add_phone_number(&elem.phone_number);
    }

    if !my_fb_profile.profile.username.is_empty() {
        profile.add_account(&format!("https://www.facebook.com/{}", my_fb_profile.profile.username), None);
    }

    // Add FB friends
    for friend_raw in my_fb_friends.iter(){
        profile.add_facebook_friend(&friend_raw.name, &friend_raw.target)
    }

    let writer = TurtleWriter::new(profile.graph.namespaces());
    let results = writer.write_to_string(&profile.graph).unwrap();
    let mut file = File::create("foo.ttl").unwrap();
    file.write_all(results.as_bytes()).unwrap();

}
