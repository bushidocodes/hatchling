use std::env;
use std::process;

use solidprofileimporter::facebook::FBProfileInformation;
use solidprofileimporter::solidgraph::open_solid_card;
use solidprofileimporter::Config;

use rdf::writer::turtle_writer::TurtleWriter;
use rdf::writer::rdf_writer::RdfWriter;
use rdf::graph::Graph;
use rdf::uri::Uri;
use rdf::triple::Triple;
use rdf::namespace::Namespace;

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

    println!("Got a name? {}", &my_fb_profile.profile.name.full_name);


    // open_solid_card(&config.solid_profile_card);


    let mut graph = Graph::new(None);

    // Add Namespaces
    graph.add_namespace(&Namespace::new("".to_string(), Uri::new("#".to_string())));


    graph.add_namespace(&Namespace::new("profile".to_string(), Uri::new("./".to_string())));
        
    graph.add_namespace(&Namespace::new("schema".to_string(), Uri::new("http://schema.org/".to_string())));
    graph.add_namespace(&Namespace::new("foaf".to_string(), Uri::new("http://xmlns.com/foaf/0.1/".to_string())));
    graph.add_namespace(&Namespace::new("vcard".to_string(), Uri::new("http://www.w3.org/2006/vcard/ns".to_string())));

    // Add foaf Metadata
    let subject = graph.create_uri_node(&Uri::new(":card".to_string()));
    let predicate = graph.create_uri_node(&Uri::new("a".to_string()));
    let object = graph.create_uri_node(&Uri::new("foaf:PersonalProfileDocument".to_string()));
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);

    let predicate = graph.create_uri_node(&Uri::new("foaf:maker".to_string()));
    let object = graph.create_uri_node(&Uri::new(":me".to_string()));
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);

    let predicate = graph.create_uri_node(&Uri::new("foaf:primaryTopic".to_string()));
    let object = graph.create_uri_node(&Uri::new(":me".to_string()));
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);
    
    
    
    // Create Me
    let subject = graph.create_uri_node(&Uri::new(":me".to_string()));
    let predicate = graph.create_uri_node(&Uri::new("a".to_string()));
    let object = graph.create_uri_node(&Uri::new("schema:Person".to_string()));
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);

    let subject = graph.create_uri_node(&Uri::new(":me".to_string()));
    let predicate = graph.create_uri_node(&Uri::new("a".to_string()));
    let object = graph.create_uri_node(&Uri::new("foaf:Person".to_string()));
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);


    // foaf familyName / lastName
    let predicate = graph.create_uri_node(&Uri::new("foaf:familyName".to_string()));
    let object = graph.create_literal_node(my_fb_profile.profile.name.last_name.clone());
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);
    let predicate = graph.create_uri_node(&Uri::new("foaf:lastName".to_string()));
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);

    // foaf givenName / firstName     
    let predicate = graph.create_uri_node(&Uri::new("foaf:givenName".to_string()));
    let object = graph.create_literal_node(my_fb_profile.profile.name.first_name.clone());
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);
    let predicate = graph.create_uri_node(&Uri::new("foaf:firstName".to_string()));
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);

    // foaf gender
    let predicate = graph.create_uri_node(&Uri::new("foaf:gender".to_string()));
    let object = graph.create_literal_node(my_fb_profile.profile.gender.gender_option);
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);

    // foaf birthday
    let predicate = graph.create_uri_node(&Uri::new("foaf:birhtday".to_string()));
    let birthday = format!("{}-{}", &my_fb_profile.profile.birthday.month, &my_fb_profile.profile.birthday.day);
    let object = graph.create_literal_node(birthday);
    let triple = Triple::new(&subject, &predicate, &object);
    graph.add_triple(&triple);



    let writer = TurtleWriter::new(&graph.namespaces());
    println!("{}", writer.write_to_string(&graph).unwrap());

}
