// Config parses CLI arguments
// Expects a path to a Facebook zip file containing JSON
pub struct Config {
  pub facebook_zip: String,
  pub solid_profile_card: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Need a path to a Facebook zip! and a URL to a SOLID profile card");
    }
    let facebook_zip = args[1].clone();
    let solid_profile_card = args[2].clone();

    Ok(Config { facebook_zip, solid_profile_card })
  }
}

pub mod solidgraph {
  use rdf::graph::Graph;
  use rdf::uri::Uri;
  use rdf::triple::Triple;
  use std::fs::File;
  use std::io::prelude::*;
  use rdf::reader::turtle_parser::TurtleParser;
  use rdf::reader::rdf_parser::RdfParser;

  // Used to open an existing .ttl profile to append new data. Parser seems to choke on SOLID's default profile
  pub fn open_solid_card(path: &str) {
    let mut file = File::open(path).unwrap_or_else(|err| panic!("Problem opening the file: {:?}", err));
      let mut solid_profile_card = String::new();
      file.read_to_string(&mut solid_profile_card);
      println!("Got a card? {:#?}", solid_profile_card);
      println!();
      
      let mut reader = TurtleParser::from_string(solid_profile_card.to_string());
      
      match reader.decode(){
        Ok(graph)=> {
          println!("{:#?}", graph);
        },
        Err(err) => println!("{}", err),
      }

  }
}

pub mod facebook {
  use serde::Deserialize;
  use std::fs::File;
  use std::io;
  use std::io::prelude::Read;

  #[derive(Deserialize)]
  pub struct FBProfileInformation {
    pub profile: Profile,
  }

  #[derive(Deserialize)]
  pub struct Profile {
    pub name: Name,
    pub emails: Emails,
    pub birthday: Date,
    pub gender: Gender,
    pub previous_names: Vec<TimestampedString>,
    // Unclear on how other_names works
    pub current_city: TimestampedString,
    pub hometown: TimestampedString,
    pub relationship: Relationship,
    pub family_members: Vec<FamilyMember>,
    pub education_experiences: Vec<EducationExperience>,
    pub work_experiences: Vec<WorkExperience>,
    pub languages: Vec<String>,
    pub political_view: View,
    pub religious_view: View,
    pub professional_skills: Vec<String>,
    pub address: Address,
    pub phone_numbers: Vec<PhoneNumber>,
    pub username: String,
    pub places_lived: Vec<PlaceLived>,
    // Ignore Favorite Quotes
    pub name_pronunciation: String,
    // Ignore Pages
    // Ignore Groups
    pub profile_uri: String, // Is there a URL type?
    pub intro_bio: String,
  }
  impl FBProfileInformation {
    pub fn new(zip_path: &str) -> Result<FBProfileInformation, io::Error> {
      let file =
        File::open(zip_path).unwrap_or_else(|err| panic!("Problem opening the file: {:?}", err));

      let mut zip = zip::ZipArchive::new(file)?;
      println!("Got a zip with {} files", zip.len());

      // The path seems to be able to be either facebook-bushidocodes/profile_information/profile_information.json
      // or profile_information/profile_information.json
      let mut file =
        zip.by_name("profile_information/profile_information.json")?;
      println!("Got file in zip");

      let mut contents = String::new();
      file
        .read_to_string(&mut contents)
        .expect("Failed to read file to string");

      let p: FBProfileInformation = serde_json::from_str(&contents).expect("error parsing JSON!");
      Ok(p)
    }
  }

  #[derive(Deserialize)]
  pub struct Name {
    pub full_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
  }
  #[derive(Deserialize)]
  pub struct Emails {
    pub emails: Vec<String>,
    pub previous_emails: Vec<String>,
    pub pending_emails: Vec<String>,
    pub ad_account_emails: Vec<String>,
  }
  #[derive(Deserialize)]
  pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
  }
  #[derive(Deserialize)]
  pub struct Gender {
    pub gender_option: String,
    pub pronoun: String,
  }
  #[derive(Deserialize)]
  pub struct TimestampedString {
    pub name: String,
    pub timestamp: u64,
  }
  #[derive(Deserialize)]
  pub struct Relationship {
    pub status: String,
    pub partner: String,
    pub anniversary: Date,
  }
  #[derive(Deserialize)]
  pub struct FamilyMember {
    pub name: String,
    pub relation: String,
  }

  #[derive(Deserialize)]
  pub struct EducationExperience {
    pub name: String,
    #[serde(default)]
    pub start_timestamp: u64,
    #[serde(default)]
    pub end_timestamp: u64,
    pub graduated: bool,
    #[serde(default)]
    pub description: String,
    pub concentrations: Vec<String>,
    #[serde(default)]
    pub degree: String,
    pub school_type: String,
  }
  // #[derive(Deserialize)]
  // pub enum EducationExperience {
  //   HighSchool {
  //     name: String,
  //     start_timestamp: u64,
  //     end_timestamp: u64,
  //     graduated: bool,
  //     description: String,
  //   },
  //   College {
  //     name: String,
  //     start_timestamp: u64,
  //     end_timestamp: u64,
  //     graduated: bool,
  //     description: String,
  //     concentrations: Vec<String>, // Up to three?
  //   },
  //   GraduateSchool {
  //     name: String,
  //     start_timestamp: u64,
  //     end_timestamp: u64,
  //     graduated: bool,
  //     description: String,
  //     concentrations: Vec<String>,
  //     degree: String,
  //   },
  // }

  // TODO: Determine how "I currently work here" is encoded
  #[derive(Deserialize)]
  pub struct WorkExperience {
    pub employer: String,
    pub title: String,
    #[serde(default)]
    pub location: String,
    pub description: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
  }

  // Used for political and religious views
  #[derive(Deserialize)]
  pub struct View {
    pub name: String,
    pub description: String,
  }
  #[derive(Deserialize)]
  pub struct Address {
    pub street: String,
    pub city: String,
    pub zipcode: String,
    pub neighborhood: String,
    pub country: String,
    pub country_code: String,
    pub region: String,
  }
  #[derive(Deserialize)]
  pub struct PhoneNumber {
    pub phone_type: String,
    pub phone_number: String,
    pub verified: bool,
  }
  #[derive(Deserialize)]
  pub struct PlaceLived {
    pub place: String,
    pub start_timestamp: u64, // No clear end_timestamp
  }
}
