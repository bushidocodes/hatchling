// use chrono::prelude::*;
// use chrono::Utc;
use rdf::graph::Graph;
use rdf::namespace::Namespace;
use rdf::triple::Triple;
use rdf::uri::Uri;
use rdf::writer::rdf_writer::RdfWriter;
use rdf::writer::turtle_writer::TurtleWriter;

pub fn clean_string(src: &str) -> String {
    src.replace(" ", "_")
        .replace(".", "_")
        .replace("-", "_")
        .replace(",", "")
}

pub struct Profile {
    pub graph: rdf::graph::Graph,
}

impl Profile {
    pub fn new() -> Profile {
        let mut new_profile = Profile {
            graph: Graph::new(None),
        };
        new_profile
            .graph
            .add_namespace(&Namespace::new("".to_string(), Uri::new("#".to_string())));
        new_profile.graph.add_namespace(&Namespace::new(
            "profile".to_string(),
            Uri::new("./".to_string()),
        ));
        new_profile.graph.add_namespace(&Namespace::new(
            "schema".to_string(),
            Uri::new("http://schema.org/".to_string()),
        ));
        new_profile.graph.add_namespace(&Namespace::new(
            "foaf".to_string(),
            Uri::new("http://xmlns.com/foaf/0.1/".to_string()),
        ));
        new_profile.graph.add_namespace(&Namespace::new(
            "vcard".to_string(),
            Uri::new("http://www.w3.org/2006/vcard/ns".to_string()),
        ));
        new_profile.graph.add_namespace(&Namespace::new(
            "resource".to_string(),
            Uri::new("http://dbpedia.org/resource/".to_string()),
        ));

        let foaf_maker = new_profile
            .graph
            .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/maker".to_string()));

        let foaf_person = new_profile
            .graph
            .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/Person".to_string()));
        let foaf_personal_profile_document = new_profile.graph.create_uri_node(&Uri::new(
            "http://xmlns.com/foaf/0.1/PersonalProfileDocument".to_string(),
        ));
        let foaf_primary_topic = new_profile.graph.create_uri_node(&Uri::new(
            "http://xmlns.com/foaf/0.1/primaryTopic".to_string(),
        ));

        let schema_person = new_profile
            .graph
            .create_uri_node(&Uri::new("http://schema.org/Person".to_string()));

        //  The <> (the empty URI) means "this document".
        let solid_card = new_profile.graph.create_uri_node(&Uri::new("".to_string()));

        let me = new_profile
            .graph
            .create_uri_node(&Uri::new("#me".to_string()));
        let is_a = new_profile
            .graph
            .create_uri_node(&Uri::new("a".to_string()));
        new_profile.graph.add_triple(&Triple::new(
            &solid_card,
            &is_a,
            &foaf_personal_profile_document,
        ));
        new_profile
            .graph
            .add_triple(&Triple::new(&solid_card, &foaf_maker, &me));
        new_profile
            .graph
            .add_triple(&Triple::new(&solid_card, &foaf_primary_topic, &me));
        new_profile
            .graph
            .add_triple(&Triple::new(&me, &is_a, &schema_person));
        new_profile
            .graph
            .add_triple(&Triple::new(&me, &is_a, &foaf_person));
        new_profile
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_name(&mut self, name: &str) {
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/name".to_string())),
            &self.graph.create_literal_node(name.to_string()),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/name".to_string())),
            &self.graph.create_literal_node(name.to_string()),
        ));
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_last_name(&mut self, lastname: &str) {
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/lastName".to_string())),
            &self.graph.create_literal_node(lastname.to_string()),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self.graph.create_uri_node(&Uri::new(
                "http://xmlns.com/foaf/0.1/familyName".to_string(),
            )),
            &self.graph.create_literal_node(lastname.to_string()),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/familyName".to_string())),
            &self.graph.create_literal_node(lastname.to_string()),
        ));
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_first_name(&mut self, firstname: &str) {
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/firstName".to_string())),
            &self.graph.create_literal_node(firstname.to_string()),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/givenName".to_string())),
            &self.graph.create_literal_node(firstname.to_string()),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/givenName".to_string())),
            &self.graph.create_literal_node(firstname.to_string()),
        ));
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_gender(&mut self, gender: &str) {
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/gender".to_string())),
            &self.graph.create_literal_node(gender.to_string()),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/gender".to_string())),
            &self.graph.create_literal_node(gender.to_string()),
        ));
    }

    // Note, this would set multiple conflicting triples if executed multiple times
    pub fn set_birthday_and_age(&mut self, month: u32, day: u32, year: i32) {
        let me = self.graph.create_uri_node(&Uri::new("#me".to_string()));
        self.graph.add_triple(&Triple::new(
            &me,
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/birthday".to_string())),
            &self
                .graph
                .create_literal_node(format!("{}-{}", &month, &day)),
        ));
        // Uses ISO 8601 https://en.wikipedia.org/wiki/ISO_8601
        self.graph.add_triple(&Triple::new(
            &me,
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/birthDate".to_string())),
            &self
                .graph
                .create_literal_node(format!("{}-{}-{}", &year, &month, &day)),
        ));

        // Calculate age. This seems to be kinda broken.
        // let utc_birthday = Utc.ymd(year, month, day).and_hms(0, 0, 0);
        // let age = Utc::now().signed_duration_since(utc_birthday).num_weeks() / 52;
        // self.graph.add_triple(&Triple::new(
        //     &me,
        //     &self
        //         .graph
        //         .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/age".to_string())),
        //     &self.graph.create_literal_node(age.to_string()),
        // ));
    }

    pub fn add_phone_number(&mut self, phonenum: &str) {
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/phone".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new(format!("tel:{}", phonenum))),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/telephone".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new(format!("tel:{}", phonenum))),
        ));
    }

    pub fn add_birth_place(&mut self, birth_place: &str) {
        let birth_place_node = &self
            .graph
            .create_blank_node_with_id(clean_string(birth_place));

        self.graph.add_triple(&Triple::new(
            &birth_place_node,
            &self.graph.create_uri_node(&Uri::new("a".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/Place".to_string())),
        ));
        self.graph.add_triple(&Triple::new(
            &birth_place_node,
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/address".to_string())),
            &self.graph.create_literal_node(birth_place.to_string()),
        ));

        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/birthPlace".to_string())),
            &birth_place_node,
        ));
    }

    pub fn add_home_location(&mut self, home_location: &str) {
        let home_location_node = &self
            .graph
            .create_blank_node_with_id(clean_string(home_location));

        self.graph.add_triple(&Triple::new(
            &home_location_node,
            &self.graph.create_uri_node(&Uri::new("a".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/Place".to_string())),
        ));
        self.graph.add_triple(&Triple::new(
            &home_location_node,
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/address".to_string())),
            &self.graph.create_literal_node(home_location.to_string()),
        ));

        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/homeLocation".to_string())),
            &home_location_node,
        ));
    }

    pub fn add_email(&mut self, email: &str) {
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/mbox".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new(format!("mailto:{}", email))),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/email".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new(format!("mailto:{}", email))),
        ));
    }

    pub fn add_alumni_relationship(&mut self, school_name: &str) {
        let school = &self
            .graph
            .create_blank_node_with_id(clean_string(school_name));
        self.graph.add_triple(&Triple::new(
            &school,
            &self.graph.create_uri_node(&Uri::new("a".to_string())),
            &self.graph.create_uri_node(&Uri::new(
                "http://schema.org/EducationalOrganization".to_string(),
            )),
        ));
        self.graph.add_triple(&Triple::new(
            &school,
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/name".to_string())),
            &self.graph.create_literal_node(school_name.to_string()),
        ));

        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/alumniOf".to_string())),
            &school,
        ));
    }

    // username is a string containing the facebook username
    // target is an option containing a
    pub fn add_account(&mut self, username: &str, account_holder_id_override: Option<&str>) {
        let mut account_holder_id = "me";
        if let Some(id) = account_holder_id_override {
            account_holder_id = id;
        }
        let account_holder_id_pragma = format!("#{}", account_holder_id);

        // Associate with my foaf profile
        self.graph.add_triple(&Triple::new(
            &self
                .graph
                .create_uri_node(&Uri::new(account_holder_id_pragma.to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/account".to_string())),
            &self.graph.create_literal_node(username.to_string()),
        ));
    }

    pub fn add_facebook_friend(&mut self, name: &str, fb_profile_url: &str) {
        let friend = self
            .graph
            .create_uri_node(&Uri::new(format!("#{}", clean_string(name))));

        self.graph.add_triple(&Triple::new(
            &friend,
            &self.graph.create_uri_node(&Uri::new("a".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/Person".to_string())),
        ));

        self.graph.add_triple(&Triple::new(
            &friend,
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/name".to_string())),
            &self.graph.create_literal_node(name.to_string()),
        ));

        self.add_account(fb_profile_url, Some(&clean_string(name)));

        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/knows".to_string())),
            &friend,
        ));
    }
    pub fn write_to_string(&mut self) -> Result<String, rdf::error::Error> {
        let writer = TurtleWriter::new(self.graph.namespaces());
        writer.write_to_string(&self.graph)
    }
}
