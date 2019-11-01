// Config parses CLI arguments
// Expects a path to a Facebook zip file containing JSON
pub struct Config {
  pub facebook_zip: String,
  pub friends_json: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Need a path to a Facebook zip! and a URL to a SOLID profile card");
    }
    let facebook_zip = args[1].clone();
    let friends_json = args[2].clone();

    Ok(Config { facebook_zip, friends_json })
  }
}

pub mod solidprofile {
  use chrono::prelude::*;
  use chrono::{Utc};
  use rdf::graph::Graph;
  use rdf::uri::Uri;
  use rdf::triple::Triple;
  use rdf::namespace::Namespace; 

  pub struct Profile {
    pub graph: rdf::graph::Graph,
  }

  impl Profile  {
    pub fn new() -> Profile  {
      // Instantiate our RDF graph
      let mut new_profile = Profile {
        graph: Graph::new(None),
      };
      
      // Add Namespaces
      new_profile.graph.add_namespace(&Namespace::new("".to_string(), Uri::new("#".to_string())));
      new_profile.graph.add_namespace(&Namespace::new("profile".to_string(), Uri::new("./".to_string())));    
      new_profile.graph.add_namespace(&Namespace::new("schema".to_string(), Uri::new("http://schema.org/".to_string())));
      new_profile.graph.add_namespace(&Namespace::new("foaf".to_string(), Uri::new("http://xmlns.com/foaf/0.1/".to_string())));
      new_profile.graph.add_namespace(&Namespace::new("vcard".to_string(), Uri::new("http://www.w3.org/2006/vcard/ns".to_string())));
      new_profile.graph.add_namespace(&Namespace::new("resource".to_string(), Uri::new("https://dbpedia.org/resource/".to_string())));

      let foaf_maker = new_profile.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/maker".to_string()));
      
      let foaf_person = new_profile.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/Person".to_string()));
      let foaf_personal_profile_document = new_profile.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/PersonalProfileDocument".to_string()));
      let foaf_primary_topic = new_profile.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/primaryTopic".to_string()));

      let schema_person = new_profile.graph.create_uri_node(&Uri::new("http://schema.org/Person".to_string()));

      // Decorate with foaf schema nodes

      //  The <> (the empty URI) means "this document".
      let solid_card = new_profile.graph.create_uri_node(&Uri::new("".to_string()));
      
      let me = new_profile.graph.create_uri_node(&Uri::new("#me".to_string()));
      let is_a = new_profile.graph.create_uri_node(&Uri::new("a".to_string()));

      
      // This is a foaf personal profile I made for myself
      new_profile.graph.add_triple(&Triple::new(&solid_card, &is_a, &foaf_personal_profile_document));
      new_profile.graph.add_triple(&Triple::new(&solid_card, &foaf_maker, &me));
      new_profile.graph.add_triple(&Triple::new(&solid_card, &foaf_primary_topic, &me));
        
      // Create Me
      new_profile.graph.add_triple(&Triple::new(&me, &is_a, &schema_person));  // I'm a Schema.org Person
      new_profile.graph.add_triple(&Triple::new(&me, &is_a, &foaf_person));    // and a foaf Person

      
      // And return
      new_profile
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_name(&mut self, name: &str){
      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new("#me".to_string())),
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/name".to_string())), 
        &self.graph.create_literal_node(name.to_string())
      ));
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_last_name(&mut self, lastname: &str){
      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new("#me".to_string())),
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/lastName".to_string())), 
        &self.graph.create_literal_node(lastname.to_string())
      ));
      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new("#me".to_string())),
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/familyName".to_string())), 
        &self.graph.create_literal_node(lastname.to_string())
      ));
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_first_name(&mut self, firstname: &str){
      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new("#me".to_string())),
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/firstName".to_string())), 
        &self.graph.create_literal_node(firstname.to_string())
      ));
      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new("#me".to_string())),
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/givenName".to_string())), 
        &self.graph.create_literal_node(firstname.to_string())
      ));
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_gender(&mut self, gender: &str){
      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new("#me".to_string())),
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/gender".to_string())), 
        &self.graph.create_literal_node(gender.to_string()),
      ));
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_birthday_and_age(&mut self, month: u32, day: u32, year: i32){
      let me = self.graph.create_uri_node(&Uri::new("#me".to_string()));
      self.graph.add_triple(&Triple::new(
        &me, 
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/birthday".to_string())), 
        &self.graph.create_literal_node(format!("{}-{}", &month, &day))
      ));

      // Calculate age. This seems to be kinda broken.
      let utc_birthday = Utc.ymd(year, month, day).and_hms(0, 0, 0);
      let age = Utc::now().signed_duration_since(utc_birthday).num_weeks() / 52;
      
      self.graph.add_triple(&Triple::new(
        &me, 
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/age".to_string())), 
        &self.graph.create_literal_node(age.to_string())
      ));        
    }

    pub fn add_phone_number(&mut self, phonenum: &str){
      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new("#me".to_string())),
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/phone".to_string())), 
        &self.graph.create_uri_node(&Uri::new(format!("tel:{}", phonenum)))
      ));
    }

    // username is a string containing the facebook username
    // target is an option containing a 
    pub fn add_account(&mut self, username: &str, account_holder_id_override: Option<&str>){
      let mut account_holder_id = "me";
      if let Some(id) = account_holder_id_override {
        account_holder_id = id;
      }
      let account_holder_id_pragma = format!("#{}", account_holder_id);

      // Associate with my foaf profile
      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new(account_holder_id_pragma.to_string())), 
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/account".to_string())), 
        &self.graph.create_literal_node(username.to_string())
      ));
    }

    pub fn add_facebook_friend(&mut self, name: &str, fb_profile_url: &str){

      let friend_id = name.replace(" ", "_");
      let friend_id = friend_id.replace(".", "_");
      let friend_id = friend_id.replace("-", "_");
      // let friend = self.graph.create_blank_node_with_id(id: String)

      let friend = self.graph.create_uri_node(&Uri::new(format!("#{}", friend_id)));

      self.graph.add_triple(&Triple::new(
        &friend, 
        &self.graph.create_uri_node(&Uri::new("a".to_string())), 
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/Person".to_string()))
      ));

      self.graph.add_triple(&Triple::new(
        &friend, 
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/name".to_string())), 
        &self.graph.create_literal_node(name.to_string())
      ));

      self.add_account(
        fb_profile_url,
        Some(&friend_id)
      );

      self.graph.add_triple(&Triple::new(
        &self.graph.create_uri_node(&Uri::new("#me".to_string())),
        &self.graph.create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/knows".to_string())), 
        &friend
      ));
    }
  }
}

pub mod facebook {
  use serde::Deserialize;
  use std::fs::File;
  use std::io;
  use std::io::prelude::Read;

  #[derive(Deserialize)]
  pub struct FBFriends {
    pub friends: Vec<FBFriend>
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
      file
        .read_to_string(&mut contents)
        .expect("Failed to read file to string");

      let p: Vec<FBFriend> = serde_json::from_str(&contents).expect("error parsing JSON!");
      Ok(p)
    }
  }

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
