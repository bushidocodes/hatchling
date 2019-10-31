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

    // let my_fb_friends = FBFriends::new(&config.friends_json).unwrap_or_else(|err| {
    //     eprintln!("Problem Parsing friends json: {}", err);
    //     process::exit(1)
    // });

    let mut profile = Profile::new();

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
        profile.add_facebook_username(&my_fb_profile.profile.username);
    }

  


    // Add FB friends
    
    // for friend_raw in my_fb_friends.iter(){
    //     let friend = graph.create_uri_node(&Uri::new(format!(":{}", friend_raw.name.replace(" ", "_"))));

    //     graph.add_triple(&Triple::new(&friend, &is_a, &foaf_person));

    //     graph.add_triple(&Triple::new(&friend, &is_a, &foaf_person));

    //     let words_in_name:  Vec<_> = friend_raw.name.split_whitespace().collect();
    //     let first_name = words_in_name[0];
    //     let last_name = words_in_name[words_in_name.len() -1];
    //     let last_name = graph.create_literal_node(last_name.to_string());
    //     let first_name = graph.create_literal_node(first_name.to_string());

    //     graph.add_triple(&Triple::new(&friend, &foaf_last_name, &last_name));

    //     graph.add_triple(&Triple::new(&friend, &foaf_first_name, &first_name));

    //     graph.add_triple(&Triple::new(&me, &foaf_knows, &friend));
    //     // friend fb account
    //     let friend_fb = graph.create_uri_node(&Uri::new(format!(":{}_fb", friend_raw.name.replace(" ", "_"))));

    // }

    let writer = TurtleWriter::new(profile.graph.namespaces());
    let results = writer.write_to_string(&profile.graph).unwrap();
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(results.as_bytes()).unwrap();

}
