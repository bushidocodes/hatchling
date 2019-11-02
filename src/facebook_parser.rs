use serde::Deserialize;
use std::fs::File;
use std::io;
use std::io::prelude::Read;

#[derive(Deserialize)]
pub struct FBFriends {
    pub friends: Vec<FBFriend>,
}

#[derive(Deserialize)]
pub struct FBFriend {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub target: String,
}

impl FBFriends {
    pub fn new(path: &str) -> Result<Vec<FBFriend>, io::Error> {
        let mut file =
            File::open(path).unwrap_or_else(|err| panic!("Problem opening the file: {:?}", err));

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file to string");

        let p: Vec<FBFriend> = serde_json::from_str(&contents).expect("error parsing JSON!");
        Ok(p)
    }
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: Name,
    pub emails: Emails,
    pub birthday: Date,
    pub gender: Gender,
    pub previous_names: Vec<TimestampedString>,
    pub current_city: TimestampedString,
    pub hometown: TimestampedString,
    pub relationship: Relationship,
    pub family_members: Vec<FamilyMember>,
    pub education_experiences: Vec<EducationExperience>,
    pub work_experiences: Vec<WorkExperience>,
    pub languages: Vec<String>,
    pub political_view: View,
    pub religious_view: View,
    #[serde(default)]
    pub professional_skills: Vec<String>,
    pub address: Address,
    pub phone_numbers: Vec<PhoneNumber>,
    #[serde(default)]
    pub username: String,
    pub places_lived: Vec<PlaceLived>,
    #[serde(default)]
    pub name_pronunciation: String,
    #[serde(default)]
    pub profile_uri: String, // Is there a URL type?
    #[serde(default)]
    pub intro_bio: String,
}

#[derive(Deserialize, Debug)]
pub struct FBProfileInformation {
    pub profile: Profile,
}

impl FBProfileInformation {
    pub fn new(path: &str) -> Result<FBProfileInformation, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let p: FBProfileInformation = serde_json::from_str(&contents)?;
        Ok(p)
    }
}

#[derive(Deserialize, Debug)]
pub struct Name {
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub middle_name: String,
    #[serde(default)]
    pub last_name: String,
}
#[derive(Deserialize, Debug)]
pub struct Emails {
    #[serde(default)]
    pub emails: Vec<String>,
    #[serde(default)]
    pub previous_emails: Vec<String>,
    #[serde(default)]
    pub pending_emails: Vec<String>,
    #[serde(default)]
    pub ad_account_emails: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct Date {
    #[serde(default)]
    pub year: u16,
    #[serde(default)]
    pub month: u8,
    #[serde(default)]
    pub day: u8,
}
#[derive(Deserialize, Debug)]
pub struct Gender {
    #[serde(default)]
    pub gender_option: String,
    #[serde(default)]
    pub pronoun: String,
}
#[derive(Deserialize, Debug)]
pub struct TimestampedString {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub timestamp: u64,
}
#[derive(Deserialize, Debug)]
pub struct Relationship {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub partner: String,
    pub anniversary: Date,
}
#[derive(Deserialize, Debug)]
pub struct FamilyMember {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub relation: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "school_type")]
pub enum EducationExperience {
    #[serde(rename = "High School")]
    HighSchool {
        #[serde(default)]
        name: String,
        #[serde(default)]
        start_timestamp: u64,
        #[serde(default)]
        end_timestamp: u64,
        #[serde(default)]
        graduated: bool,
        #[serde(default)]
        description: String,
    },
    College {
        #[serde(default)]
        name: String,
        #[serde(default)]
        start_timestamp: u64,
        #[serde(default)]
        end_timestamp: u64,
        #[serde(default)]
        graduated: bool,
        #[serde(default)]
        description: String,
        #[serde(default)]
        concentrations: Vec<String>, // Up to three?
    },
    #[serde(rename = "Graduate School")]
    GraduateSchool {
        #[serde(default)]
        name: String,
        #[serde(default)]
        start_timestamp: u64,
        #[serde(default)]
        end_timestamp: u64,
        graduated: bool,
        #[serde(default)]
        description: String,
        #[serde(default)]
        concentrations: Vec<String>,
        #[serde(default)]
        degree: String,
    },
}

// TODO: Determine how "I currently work here" is encoded
#[derive(Deserialize, Debug)]
pub struct WorkExperience {
    #[serde(default)]
    pub employer: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub start_timestamp: u64,
    #[serde(default)]
    pub end_timestamp: u64,
}

// Used for political and religious views
#[derive(Deserialize, Debug)]
pub struct View {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}
#[derive(Deserialize, Debug)]
pub struct Address {
    #[serde(default)]
    pub street: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub zipcode: String,
    #[serde(default)]
    pub neighborhood: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub region: String,
}
#[derive(Deserialize, Debug)]
pub struct PhoneNumber {
    #[serde(default)]
    pub phone_type: String,
    #[serde(default)]
    pub phone_number: String,
    #[serde(default)]
    pub verified: bool,
}
#[derive(Deserialize, Debug)]
pub struct PlaceLived {
    #[serde(default)]
    pub place: String,
    #[serde(default)]
    pub start_timestamp: u64, // No clear end_timestamp
}
