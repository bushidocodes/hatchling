pub mod facebook_parser;
pub mod profile_builder;

use facebook_parser::{EducationExperience, FBFriends, FBProfileInformation};
use profile_builder::Profile;
use rdf::writer::rdf_writer::RdfWriter;
use rdf::writer::turtle_writer::TurtleWriter;
use std::io;

pub fn convert_facebook_to_solid(
    profile_path: &str,
    friends_path: Option<&str>,
) -> Result<String, io::Error> {
    let my_fb_profile = FBProfileInformation::new(profile_path)?;

    let mut profile = Profile::new();

    if !my_fb_profile.profile.name.full_name.is_empty() {
        profile.set_name(&my_fb_profile.profile.name.full_name);
    }

    if !my_fb_profile.profile.name.last_name.is_empty() {
        profile.set_last_name(&my_fb_profile.profile.name.last_name);
    }

    if !my_fb_profile.profile.name.first_name.is_empty() {
        profile.set_first_name(&my_fb_profile.profile.name.first_name);
    }

    if !my_fb_profile.profile.gender.gender_option.is_empty() {
        profile.set_gender(&my_fb_profile.profile.gender.gender_option);
    }

    if !my_fb_profile.profile.birthday.month > 0
        && !my_fb_profile.profile.birthday.day > 0
        && !my_fb_profile.profile.birthday.year > 0
    {
        profile.set_birthday_and_age(
            my_fb_profile.profile.birthday.month.into(),
            my_fb_profile.profile.birthday.day.into(),
            my_fb_profile.profile.birthday.year.into(),
        );
    }

    for elem in my_fb_profile.profile.phone_numbers.iter() {
        profile.add_phone_number(&elem.phone_number);
    }

    if !my_fb_profile.profile.username.is_empty() {
        profile.add_account(
            &format!(
                "https://www.facebook.com/{}",
                my_fb_profile.profile.username
            ),
            None,
        );
    }

    for email in my_fb_profile.profile.emails.emails {
        profile.add_email(&email);
    }

    for edu in my_fb_profile.profile.education_experiences {
        match edu {
            EducationExperience::GraduateSchool {
                name,
                graduated,
                start_timestamp: _,
                end_timestamp: _,
                description: _,
                concentrations: _,
                degree: _,
            } => {
                if graduated {
                    profile.add_alumni_relationship(&name)
                };
            }
            EducationExperience::College {
                name,
                graduated,
                start_timestamp: _,
                end_timestamp: _,
                description: _,
                concentrations: _,
            } => {
                if graduated {
                    profile.add_alumni_relationship(&name)
                };
            }
            EducationExperience::HighSchool {
                name,
                graduated,
                start_timestamp: _,
                end_timestamp: _,
                description: _,
            } => {
                if graduated {
                    profile.add_alumni_relationship(&name)
                };
            }
        }
    }

    if !my_fb_profile.profile.current_city.name.is_empty() {
        profile.add_home_location(&my_fb_profile.profile.current_city.name)
    }

    // I assume FB "hometown" maps clearly to birthPlace. This is potentially not, true
    if !my_fb_profile.profile.hometown.name.is_empty() {
        profile.add_birth_place(&my_fb_profile.profile.hometown.name)
    }

    if let Some(friends) = friends_path {
        let my_fb_friends = FBFriends::new(friends)?;
        for friend_raw in my_fb_friends.iter() {
            profile.add_facebook_friend(&friend_raw.name, &friend_raw.target)
        }
    };

    let writer = TurtleWriter::new(profile.graph.namespaces());
    let results = writer.write_to_string(&profile.graph).unwrap();
    Ok(results)
}
