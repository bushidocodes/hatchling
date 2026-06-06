use hatchling::convert_facebook_to_solid;

const PROFILE: &str = include_str!("fixtures/profile_information.json");
const FRIENDS_DYI: &str = include_str!("fixtures/your_friends.json");
const FRIENDS_SCRAPED: &str = include_str!("fixtures/friends_scraped.json");

// ---------------------------------------------------------------------------
// Profile parsing
// ---------------------------------------------------------------------------

#[test]
fn profile_parses_without_error() {
    assert!(convert_facebook_to_solid(PROFILE, None).is_ok());
}

#[test]
fn profile_name_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("Jane Doe-Smith"), "full name missing");
    assert!(ttl.contains("foaf:givenName \"Jane\""), "first name missing");
    assert!(ttl.contains("foaf:familyName \"Doe-Smith\""), "last name missing");
}

#[test]
fn profile_email_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("mailto:jane.doe@example.com"), "primary email missing");
    assert!(ttl.contains("mailto:janedoe1985@gmail.com"), "secondary email missing");
}

#[test]
fn profile_birthday_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("schema:birthDate \"1985-03-14\""), "birthDate missing");
    assert!(ttl.contains("foaf:birthday \"--03-14\""), "foaf birthday missing");
}

#[test]
fn profile_phone_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("tel:+15035550123"), "phone number missing");
}

#[test]
fn profile_locations_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("Portland, Oregon"), "homeLocation missing");
    assert!(ttl.contains("Eugene, Oregon"), "birthPlace missing");
}

#[test]
fn profile_facebook_account_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    // Account URL must be a URI node (<...>), not a literal ("...")
    assert!(
        ttl.contains("<https://www.facebook.com/jane.doe.smith.1985>"),
        "Facebook account must be a URI node"
    );
    assert!(
        !ttl.contains("\"https://www.facebook.com/jane.doe.smith.1985\""),
        "Facebook account must not be a literal"
    );
}

#[test]
fn profile_gender_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("foaf:gender \"Female\""), "foaf gender missing");
    assert!(ttl.contains("schema:gender \"Female\""), "schema gender missing");
}

// ---------------------------------------------------------------------------
// Profile page
// ---------------------------------------------------------------------------

#[test]
fn profile_page_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("foaf:page"), "foaf:page missing");
    assert!(
        ttl.contains("https://www.facebook.com/jane.doe.smith.1985"),
        "profile URI missing"
    );
}

// ---------------------------------------------------------------------------
// Work experience
// ---------------------------------------------------------------------------

/// Builds a minimal valid profile JSON with the given work_experiences value
/// substituted in, so synthetic work-experience tests don't need the full fixture.
fn profile_with_work(work_experiences_json: &str) -> String {
    const TEMPLATE: &str = r#"{"profile_v2":{"name":{"full_name":"Test User","first_name":"Test","middle_name":"","last_name":"User"},"emails":{"emails":[],"previous_emails":[],"pending_emails":[],"ad_account_emails":[]},"work_experiences":WORK}}"#;
    TEMPLATE.replace("WORK", work_experiences_json)
}

#[test]
fn work_experience_in_output() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("Acme Software Inc."), "employer missing");
    assert!(ttl.contains("Senior Software Engineer"), "job title missing");
    assert!(ttl.contains("schema:worksFor"), "worksFor predicate missing");
    assert!(ttl.contains("schema:jobTitle"), "jobTitle predicate missing");
}

#[test]
fn work_experience_employer_and_title_produce_triples() {
    let json = profile_with_work(
        r#"[{"employer":{"name":"Globex Corporation"},"title":"Safety Inspector","start_timestamp":0,"end_timestamp":0}]"#,
    );
    let ttl = convert_facebook_to_solid(&json, None).unwrap();
    assert!(ttl.contains("Globex Corporation"));
    assert!(ttl.contains("Safety Inspector"));
    assert!(ttl.contains("schema:worksFor"));
    assert!(ttl.contains("schema:jobTitle"));
}

#[test]
fn work_experience_without_title_omits_job_title_triple() {
    let json = profile_with_work(
        r#"[{"employer":{"name":"Globex Corporation"},"start_timestamp":0,"end_timestamp":0}]"#,
    );
    let ttl = convert_facebook_to_solid(&json, None).unwrap();
    assert!(ttl.contains("Globex Corporation"));
    assert!(ttl.contains("schema:worksFor"));
    assert!(!ttl.contains("schema:jobTitle"));
}

#[test]
fn work_experience_absent_omits_work_triples() {
    let json = profile_with_work("[]");
    let ttl = convert_facebook_to_solid(&json, None).unwrap();
    assert!(!ttl.contains("schema:worksFor"));
    assert!(!ttl.contains("schema:jobTitle"));
}

#[test]
fn work_experience_empty_employer_name_omits_all_work_triples() {
    let json = profile_with_work(
        r#"[{"employer":{"name":""},"title":"Engineer","start_timestamp":0,"end_timestamp":0}]"#,
    );
    let ttl = convert_facebook_to_solid(&json, None).unwrap();
    assert!(!ttl.contains("schema:worksFor"));
    assert!(!ttl.contains("schema:jobTitle"));
}

#[test]
fn work_experience_missing_employer_key_omits_all_work_triples() {
    let json = profile_with_work(
        r#"[{"title":"Engineer","start_timestamp":0,"end_timestamp":0}]"#,
    );
    let ttl = convert_facebook_to_solid(&json, None).unwrap();
    assert!(!ttl.contains("schema:worksFor"));
    assert!(!ttl.contains("schema:jobTitle"));
}

#[test]
fn work_experience_multiple_employers_produce_multiple_triples() {
    let json = profile_with_work(
        r#"[{"employer":{"name":"Acme Corp"},"title":"Coyote Wrangler"},{"employer":{"name":"Initech"},"title":"TPS Report Author"}]"#,
    );
    let ttl = convert_facebook_to_solid(&json, None).unwrap();
    assert!(ttl.contains("Acme Corp"));
    assert!(ttl.contains("Coyote Wrangler"));
    assert!(ttl.contains("Initech"));
    assert!(ttl.contains("TPS Report Author"));
    assert!(ttl.contains("schema:worksFor"));
}

// ---------------------------------------------------------------------------
// Education
// ---------------------------------------------------------------------------

#[test]
fn graduated_schools_produce_alumni_triples() {
    let ttl = convert_facebook_to_solid(PROFILE, None).unwrap();
    assert!(ttl.contains("University of Oregon"), "college missing");
    assert!(ttl.contains("Oregon State University"), "grad school missing");
    assert!(ttl.contains("Eugene High School"), "high school missing");
}

// ---------------------------------------------------------------------------
// Friends — DYI format (friends_v2, names + timestamps, no URLs)
// ---------------------------------------------------------------------------

#[test]
fn dyi_friends_parse_without_error() {
    assert!(convert_facebook_to_solid(PROFILE, Some(FRIENDS_DYI)).is_ok());
}

#[test]
fn dyi_friends_appear_in_foaf_knows() {
    let ttl = convert_facebook_to_solid(PROFILE, Some(FRIENDS_DYI)).unwrap();
    assert!(ttl.contains("foaf:knows"), "foaf:knows missing");
    assert!(ttl.contains("Alice Nguyen"), "friend missing");
    assert!(ttl.contains("Bob Kowalski"), "friend missing");
}

#[test]
fn dyi_friends_no_foaf_account_triples() {
    // DYI export has no profile URLs — no foaf:account should be emitted for friends.
    // Only the owner's own account triple should appear.
    let ttl = convert_facebook_to_solid(PROFILE, Some(FRIENDS_DYI)).unwrap();
    let account_count = ttl.matches("foaf:account").count();
    assert_eq!(account_count, 1, "expected 1 foaf:account (owner only), got {account_count}");
}

#[test]
fn dyi_friends_non_ascii_names_decoded() {
    // DYI export uses Facebook's \u00XX encoding bug — fix_facebook_encoding must handle it.
    // The fixture uses literal UTF-8 (written by the test author), so this exercises
    // the pass-through path; the broken-encoding path is tested separately below.
    let ttl = convert_facebook_to_solid(PROFILE, Some(FRIENDS_DYI)).unwrap();
    assert!(ttl.contains("Elena Petrová"),       "Petrová corrupted");
    assert!(ttl.contains("François Beaumont"),   "François corrupted");
    assert!(ttl.contains("Gabriela Müller"),     "Müller corrupted");
    assert!(ttl.contains("Ingrid Björnsdóttir"), "Björnsdóttir corrupted");
    assert!(ttl.contains("José María Vázquez"),  "José María corrupted");
}

// ---------------------------------------------------------------------------
// Friends — scraped format ([{name, target}], includes profile URLs)
// ---------------------------------------------------------------------------

#[test]
fn scraped_friends_parse_without_error() {
    assert!(convert_facebook_to_solid(PROFILE, Some(FRIENDS_SCRAPED)).is_ok());
}

#[test]
fn scraped_friends_produce_foaf_account_triples() {
    let ttl = convert_facebook_to_solid(PROFILE, Some(FRIENDS_SCRAPED)).unwrap();
    assert!(ttl.contains("https://www.facebook.com/alice.nguyen.503"), "alice URL missing");
    assert!(ttl.contains("https://www.facebook.com/francois.beaumont"), "francois URL missing");
    // Numeric profile.php?id= URL must be preserved intact
    assert!(ttl.contains("profile.php?id=100012345678"), "numeric ID URL missing");
}

#[test]
fn scraped_friends_non_ascii_names_not_corrupted() {
    // Scraped JSON contains literal UTF-8 bytes — fix_facebook_encoding must
    // pass them through without Latin-1 reinterpretation.
    let ttl = convert_facebook_to_solid(PROFILE, Some(FRIENDS_SCRAPED)).unwrap();
    assert!(ttl.contains("François Beaumont"),   "François corrupted to Ã sequence");
    assert!(ttl.contains("Gabriela Müller"),     "Müller corrupted");
    assert!(ttl.contains("Elena Petrová"),       "Petrová corrupted");
    assert!(ttl.contains("Ingrid Björnsdóttir"), "Björnsdóttir corrupted");
    assert!(ttl.contains("José María Vázquez"),  "José María corrupted");
}

// ---------------------------------------------------------------------------
// Facebook UTF-8 encoding bug fix
// ---------------------------------------------------------------------------

#[test]
fn facebook_broken_utf8_encoding_is_fixed() {
    // Facebook's encoder emits multibyte UTF-8 characters as consecutive
    // \u00XX escapes — one escape per byte instead of a single \uXXXX.
    // Example: é (U+00E9, UTF-8 bytes: 0xC3 0xA9) → Ã©
    //          è (U+00E8, UTF-8 bytes: 0xC3 0xA8) → Ã¨
    // fix_facebook_encoding() must reassemble these into the correct codepoint.
    // In a Rust raw string r#"..."# the backslash is literal, so Ã is the
    // six-character sequence that Facebook literally writes into its JSON file.
    let broken = "{\"profile_v2\":{\"name\":{\"full_name\":\"Caf\\u00c3\\u00a9 \\u00c3\\u00a9l\\u00c3\\u00a8ve\",\"first_name\":\"Caf\\u00c3\\u00a9\",\"middle_name\":\"\",\"last_name\":\"\\u00c3\\u00a9l\\u00c3\\u00a8ve\"},\"emails\":{\"emails\":[\"test@example.com\"],\"previous_emails\":[],\"pending_emails\":[],\"ad_account_emails\":[]},\"birthday\":{\"year\":0,\"month\":0,\"day\":0},\"gender\":{\"gender_option\":\"\",\"pronoun\":\"\"},\"previous_names\":[],\"family_members\":[],\"education_experiences\":[],\"work_experiences\":[],\"languages\":[],\"phone_numbers\":[],\"current_city\":{\"name\":\"\",\"timestamp\":0},\"hometown\":{\"name\":\"\",\"timestamp\":0},\"relationship\":{\"status\":\"\",\"partner\":\"\",\"anniversary\":{\"year\":0,\"month\":0,\"day\":0},\"timestamp\":0},\"username\":\"\",\"profile_uri\":\"\",\"intro_bio\":{\"name\":\"\",\"timestamp\":0}}}";
    let ttl = convert_facebook_to_solid(broken, None).unwrap();
    // After the fix, Ã© → é and Ã¨ → è
    assert!(ttl.contains("Café élève"), "broken UTF-8 not repaired; expected 'Café élève'");
}
