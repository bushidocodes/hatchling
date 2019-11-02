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
    pub name: String,
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

#[derive(Deserialize, Debug)]
pub struct FBProfileInformation {
    pub profile: Profile,
}

impl FBProfileInformation {
    pub fn new(zip_path: &str) -> Result<FBProfileInformation, io::Error> {
        let file = File::open(zip_path)?;
        let mut zip = zip::ZipArchive::new(file)?;
        // The path seems to be able to be either facebook-bushidocodes/profile_information/profile_information.json
        // or profile_information/profile_information.json
        let mut file = zip.by_name("profile_information/profile_information.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let p: FBProfileInformation = serde_json::from_str(&contents)?;
        Ok(p)
    }
}

#[derive(Deserialize, Debug)]
pub struct Name {
    pub full_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
}
#[derive(Deserialize, Debug)]
pub struct Emails {
    pub emails: Vec<String>,
    pub previous_emails: Vec<String>,
    pub pending_emails: Vec<String>,
    pub ad_account_emails: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}
#[derive(Deserialize, Debug)]
pub struct Gender {
    pub gender_option: String,
    pub pronoun: String,
}
#[derive(Deserialize, Debug)]
pub struct TimestampedString {
    pub name: String,
    pub timestamp: u64,
}
#[derive(Deserialize, Debug)]
pub struct Relationship {
    pub status: String,
    pub partner: String,
    pub anniversary: Date,
}
#[derive(Deserialize, Debug)]
pub struct FamilyMember {
    pub name: String,
    pub relation: String,
}

// #[derive(Deserialize)]
// pub struct EducationExperience {
//     pub name: String,
//     #[serde(default)]
//     pub start_timestamp: u64,
//     #[serde(default)]
//     pub end_timestamp: u64,
//     pub graduated: bool,
//     #[serde(default)]
//     pub description: String,
//     pub concentrations: Vec<String>,
//     #[serde(default)]
//     pub degree: String,
//     pub school_type: String,
// }
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
    pub employer: String,
    pub title: String,
    #[serde(default)]
    pub location: String,
    pub description: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

// Used for political and religious views
#[derive(Deserialize, Debug)]
pub struct View {
    pub name: String,
    pub description: String,
}
#[derive(Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zipcode: String,
    pub neighborhood: String,
    pub country: String,
    pub country_code: String,
    pub region: String,
}
#[derive(Deserialize, Debug)]
pub struct PhoneNumber {
    pub phone_type: String,
    pub phone_number: String,
    pub verified: bool,
}
#[derive(Deserialize, Debug)]
pub struct PlaceLived {
    pub place: String,
    pub start_timestamp: u64, // No clear end_timestamp
}
