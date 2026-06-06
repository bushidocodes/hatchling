pub fn clean_string(src: &str) -> String {
    src.replace(' ', "_")
        .replace('.', "_")
        .replace('-', "_")
        .replace(',', "")
}

// ---------------------------------------------------------------------------
// Minimal RDF graph + Turtle serializer — replaces the unmaintained `rdf` crate.
// oxrdf/oxttl require absolute IRIs and cannot represent the relative-IRI
// semantics that Solid profile documents require (e.g. <>, <#me>, <./>, <#>).
// ---------------------------------------------------------------------------

struct Uri(String);

impl Uri {
    fn new(s: String) -> Self {
        Uri(s)
    }
}

#[derive(Clone)]
struct Namespace {
    prefix: String,
    iri: String,
}

impl Namespace {
    fn new(prefix: String, uri: Uri) -> Self {
        Namespace { prefix, iri: uri.0 }
    }
}

#[derive(Clone)]
enum NodeKind {
    Uri,
    Blank,
    Literal,
}

#[derive(Clone)]
struct Node {
    kind: NodeKind,
    value: String,
}

struct Triple {
    subject: Node,
    predicate: Node,
    object: Node,
}

impl Triple {
    fn new(s: &Node, p: &Node, o: &Node) -> Self {
        Triple {
            subject: s.clone(),
            predicate: p.clone(),
            object: o.clone(),
        }
    }
}

struct Graph {
    triples: Vec<Triple>,
    namespaces: Vec<Namespace>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            triples: Vec::new(),
            namespaces: Vec::new(),
        }
    }

    fn add_namespace(&mut self, ns: &Namespace) {
        self.namespaces.push(ns.clone());
    }

    fn create_uri_node(&self, uri: &Uri) -> Node {
        Node { kind: NodeKind::Uri, value: uri.0.clone() }
    }

    fn create_literal_node(&self, s: String) -> Node {
        Node { kind: NodeKind::Literal, value: s }
    }

    fn create_blank_node_with_id(&self, id: String) -> Node {
        Node { kind: NodeKind::Blank, value: id }
    }

    fn add_triple(&mut self, t: &Triple) {
        self.triples.push(Triple {
            subject: t.subject.clone(),
            predicate: t.predicate.clone(),
            object: t.object.clone(),
        });
    }

    fn format_node(&self, node: &Node) -> String {
        match node.kind {
            NodeKind::Uri => {
                let iri = &node.value;
                // "a" is the Turtle shorthand for rdf:type
                if iri == "a" {
                    return "a".to_string();
                }
                // Try namespace prefix compression
                for ns in &self.namespaces {
                    if !ns.iri.is_empty() && iri.starts_with(ns.iri.as_str()) {
                        let local = &iri[ns.iri.len()..];
                        return if ns.prefix.is_empty() {
                            format!(":{}", local)
                        } else {
                            format!("{}:{}", ns.prefix, local)
                        };
                    }
                }
                // Fall back to angle-bracket form; empty IRI means <> (this document)
                if iri.is_empty() {
                    "<>".to_string()
                } else {
                    format!("<{}>", iri)
                }
            }
            NodeKind::Blank => format!("_:{}", node.value),
            NodeKind::Literal => {
                let escaped = node
                    .value
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r");
                format!("\"{}\"", escaped)
            }
        }
    }

    fn serialize_turtle(&self) -> String {
        let mut out = String::new();
        for ns in &self.namespaces {
            out.push_str(&format!("@prefix {}: <{}> .\n", ns.prefix, ns.iri));
        }
        out.push('\n');
        for t in &self.triples {
            out.push_str(&format!(
                "{} {} {} .\n",
                self.format_node(&t.subject),
                self.format_node(&t.predicate),
                self.format_node(&t.object),
            ));
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Profile builder
// ---------------------------------------------------------------------------

pub struct Profile {
    graph: Graph,
}

impl Profile {
    pub fn new() -> Profile {
        let mut new_profile = Profile { graph: Graph::new() };

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

    pub fn set_birthday_and_age(&mut self, month: u32, day: u32, year: i32) {
        let me = self.graph.create_uri_node(&Uri::new("#me".to_string()));
        self.graph.add_triple(&Triple::new(
            &me,
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/birthday".to_string())),
            &self
                .graph
                .create_literal_node(format!("--{:02}-{:02}", &month, &day)),
        ));
        self.graph.add_triple(&Triple::new(
            &me,
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/birthDate".to_string())),
            &self
                .graph
                .create_literal_node(format!("{:04}-{:02}-{:02}", &year, &month, &day)),
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

    pub fn add_work_experience(&mut self, employer: &str, title: &str) {
        let org = &self
            .graph
            .create_blank_node_with_id(clean_string(employer));
        self.graph.add_triple(&Triple::new(
            &org,
            &self.graph.create_uri_node(&Uri::new("a".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/Organization".to_string())),
        ));
        self.graph.add_triple(&Triple::new(
            &org,
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/name".to_string())),
            &self.graph.create_literal_node(employer.to_string()),
        ));
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://schema.org/worksFor".to_string())),
            &org,
        ));
        if !title.is_empty() {
            self.graph.add_triple(&Triple::new(
                &self.graph.create_uri_node(&Uri::new("#me".to_string())),
                &self
                    .graph
                    .create_uri_node(&Uri::new("http://schema.org/jobTitle".to_string())),
                &self.graph.create_literal_node(title.to_string()),
            ));
        }
    }

    pub fn add_profile_page(&mut self, url: &str) {
        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/page".to_string())),
            &self.graph.create_uri_node(&Uri::new(url.to_string())),
        ));
    }

    pub fn add_account(&mut self, username: &str, account_holder_id_override: Option<&str>) {
        let mut account_holder_id = "me";
        if let Some(id) = account_holder_id_override {
            account_holder_id = id;
        }
        let account_holder_id_pragma = format!("#{}", account_holder_id);

        self.graph.add_triple(&Triple::new(
            &self
                .graph
                .create_uri_node(&Uri::new(account_holder_id_pragma.to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/account".to_string())),
            &self.graph.create_uri_node(&Uri::new(username.to_string())),
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

        if !fb_profile_url.is_empty() {
            self.add_account(fb_profile_url, Some(&clean_string(name)));
        }

        self.graph.add_triple(&Triple::new(
            &self.graph.create_uri_node(&Uri::new("#me".to_string())),
            &self
                .graph
                .create_uri_node(&Uri::new("http://xmlns.com/foaf/0.1/knows".to_string())),
            &friend,
        ));
    }

    pub fn write_to_string(&mut self) -> String {
        self.graph.serialize_turtle()
    }
}
