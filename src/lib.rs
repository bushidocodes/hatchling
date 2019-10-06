// Config parses CLI arguments
// Expects a path to a Facebook zip file containing JSON
pub struct Config {
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
      return Err("Need a path to a Facebook zip!");
    }
    let filename = args[1].clone();

    Ok(Config { filename })
  }
}

pub mod facebook {
  use serde::Deserialize;
  use std::fs::File;
  use std::io;
  use std::io::prelude::Read;

  #[derive(Deserialize)]
  pub struct ProfileInformation {
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
  impl ProfileInformation {
    pub fn new(zip_path: &str) -> Result<ProfileInformation, io::Error> {
      let file =
        File::open(zip_path).unwrap_or_else(|err| panic!("Problem opening the file: {:?}", err));

      let mut zip = zip::ZipArchive::new(file)?;
      println!("Got a zip with {} files", zip.len());

      let mut file =
        zip.by_name("facebook-bushidocodes/profile_information/profile_information.json")?;
      println!("Got file in zip");

      let mut contents = String::new();
      file
        .read_to_string(&mut contents)
        .expect("Failed to read file to string");

      let p: ProfileInformation = serde_json::from_str(&contents).expect("error parsing JSON!");
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
