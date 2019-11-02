use chrono::prelude::*;
use chrono::Utc;
use rdf::graph::Graph;
use rdf::namespace::Namespace;
use rdf::triple::Triple;
use rdf::uri::Uri;

pub struct Profile {
    pub graph: rdf::graph::Graph,
}

impl Profile {
    pub fn new() -> Profile {
        // Instantiate our RDF graph
        let mut new_profile = Profile {
            graph: Graph::new(None),
        };
        // Add Namespaces
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
            Uri::new("https://dbpedia.org/resource/".to_string()),
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
        // Decorate with foaf schema nodes

        //  The <> (the empty URI) means "this document".
        let solid_card = new_profile.graph.create_uri_node(&Uri::new("".to_string()));

        let me = new_profile
            .graph
            .create_uri_node(&Uri::new("#me".to_string()));
        let is_a = new_profile
            .graph
            .create_uri_node(&Uri::new("a".to_string()));
        // This is a foaf personal profile I made for myself
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
        // Create Me
        new_profile
            .graph
            .add_triple(&Triple::new(&me, &is_a, &schema_person)); // I'm a Schema.org Person
        new_profile
            .graph
            .add_triple(&Triple::new(&me, &is_a, &foaf_person)); // and a foaf Person

        // And return
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

        // Calculate age. This seems to be kinda broken.
        let utc_birthday = Utc.ymd(year, month, day).and_hms(0, 0, 0);
        let age = Utc::now().signed_duration_since(utc_birthday).num_weeks() / 52;
        self.graph.add_triple(&Triple::new(
            &me,
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/age".to_string())),
            &self.graph.create_literal_node(age.to_string()),
        ));
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
        let friend_id = name.replace(" ", "_");
        let friend_id = friend_id.replace(".", "_");
        let friend_id = friend_id.replace("-", "_");
        // let friend = self.graph.create_blank_node_with_id(id: String)

        let friend = self
            .graph
            .create_uri_node(&Uri::new(format!("#{}", friend_id)));

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

        self.add_account(fb_profile_url, Some(&friend_id));

        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/knows".to_string())),
            &friend,
        ));
    }
}
