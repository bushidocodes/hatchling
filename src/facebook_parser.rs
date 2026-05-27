// serde's Deserialize derive macro generates non-local `impl` blocks when
// combined with #[serde(tag)] / #[serde(other)] / #[serde(alias)].
// This is a known proc-macro expansion artefact, not a real code issue.
#![allow(non_local_definitions)]

use serde::Deserialize;
use serde_json::Value;
use std::io;

// --- Friends ---

#[derive(Deserialize)]
pub struct FBFriend {
    #[serde(default)]
    pub name: String,
    /// Only present in the legacy browser-scraping format.
    #[serde(default)]
    pub target: String,
    /// Present in the official "Download Your Information" export (friends_v2).
    #[serde(default)]
    pub timestamp: u64,
}

#[derive(Deserialize)]
struct FriendsV2Wrapper {
    friends_v2: Vec<FBFriend>,
}

#[derive(Deserialize)]
struct FriendsWrapper {
    friends: Vec<FBFriend>,
}

pub struct FBFriends;

impl FBFriends {
    pub fn new(contents: &str) -> Result<Vec<FBFriend>, io::Error> {
        let fixed = fix_facebook_encoding(contents);
        // Try current official DYI format: {"friends_v2": [...]}
        if let Ok(w) = serde_json::from_str::<FriendsV2Wrapper>(&fixed) {
            return Ok(w.friends_v2);
        }
        // Try older official DYI format: {"friends": [...]}
        if let Ok(w) = serde_json::from_str::<FriendsWrapper>(&fixed) {
            return Ok(w.friends);
        }
        // Fall back to the legacy browser-scraping format: [{name, target}, ...]
        let p: Vec<FBFriend> = serde_json::from_str(&fixed)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(p)
    }
}

// --- Profile ---

#[derive(Deserialize, Debug)]
pub struct FBProfileInformation {
    /// Accepts both the old "profile" key and the current "profile_v2" key.
    #[serde(alias = "profile_v2")]
    pub profile: Profile,
}

impl FBProfileInformation {
    pub fn new(contents: &str) -> Result<FBProfileInformation, io::Error> {
        let fixed = fix_facebook_encoding(contents);
        let p: FBProfileInformation = serde_json::from_str(&fixed)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(p)
    }
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: Name,
    pub emails: Emails,
    #[serde(default)]
    pub birthday: Date,
    #[serde(default)]
    pub gender: Gender,
    #[serde(default)]
    pub previous_names: Vec<Value>,
    #[serde(default)]
    pub current_city: TimestampedString,
    #[serde(default)]
    pub hometown: TimestampedString,
    #[serde(default)]
    pub relationship: Relationship,
    #[serde(default)]
    pub family_members: Vec<FamilyMember>,
    #[serde(default)]
    pub education_experiences: Vec<EducationExperience>,
    /// Stored as raw JSON because the field structure changed between export versions.
    #[serde(default)]
    pub work_experiences: Vec<Value>,
    /// Stored as raw JSON because the field structure changed between export versions
    /// (was Vec<String>, now Vec<{name, timestamp}>).
    #[serde(default)]
    pub languages: Vec<Value>,
    #[serde(default)]
    pub political_view: View,
    #[serde(default)]
    pub religious_view: View,
    #[serde(default)]
    pub professional_skills: Vec<String>,
    #[serde(default)]
    pub address: Address,
    #[serde(default)]
    pub phone_numbers: Vec<PhoneNumber>,
    #[serde(default)]
    pub username: String,
    /// Stored as raw JSON because the nested structure changed between export versions.
    #[serde(default)]
    pub places_lived: Vec<Value>,
    #[serde(default)]
    pub name_pronunciation: String,
    #[serde(default)]
    pub profile_uri: String,
    /// Stored as raw JSON because the field changed from a plain string to
    /// an object {name, timestamp} in newer exports.
    #[serde(default)]
    pub intro_bio: Value,
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

#[derive(Deserialize, Debug, Default)]
pub struct Date {
    #[serde(default)]
    pub year: u16,
    #[serde(default)]
    pub month: u8,
    #[serde(default)]
    pub day: u8,
}

#[derive(Deserialize, Debug, Default)]
pub struct Gender {
    #[serde(default)]
    pub gender_option: String,
    #[serde(default)]
    pub pronoun: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct TimestampedString {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct Relationship {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub partner: String,
    #[serde(default)]
    pub anniversary: Date,
    #[serde(default)]
    pub timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct FamilyMember {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub relation: String,
    #[serde(default)]
    pub timestamp: u64,
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
        concentrations: Vec<String>,
    },
    #[serde(rename = "Graduate School")]
    GraduateSchool {
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
        concentrations: Vec<String>,
        #[serde(default)]
        degree: String,
    },
    /// Catch-all for any school_type values not listed above.
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug, Default)]
pub struct View {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Deserialize, Debug, Default)]
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

// --- Encoding fix ---

/// Facebook's JSON encoder has a bug: UTF-8 multi-byte characters are output as
/// consecutive \u00XX escape sequences (one per byte) instead of a single \uXXXX
/// sequence. For example, 'é' (U+00E9, UTF-8: 0xC3 0xA9) appears as Ã©
/// instead of é. This function detects and corrects those sequences in the
/// raw JSON string before it is handed to serde_json.
fn fix_facebook_encoding(input: &str) -> String {
    let s = input.as_bytes();
    let len = s.len();
    let mut result = String::with_capacity(len);
    let mut i = 0;

    while i < len {
        // Detect the start of a \u00XX pattern (6 bytes: \, u, 0, 0, H, H).
        if i + 5 < len
            && s[i] == b'\\'
            && s[i + 1] == b'u'
            && s[i + 2] == b'0'
            && s[i + 3] == b'0'
            && is_hex(s[i + 4])
            && is_hex(s[i + 5])
        {
            let seq_start = i;
            let mut buf: Vec<u8> = Vec::new();

            // Collect all consecutive \u00XX sequences.
            while i + 5 < len
                && s[i] == b'\\'
                && s[i + 1] == b'u'
                && s[i + 2] == b'0'
                && s[i + 3] == b'0'
                && is_hex(s[i + 4])
                && is_hex(s[i + 5])
            {
                buf.push((hex_digit(s[i + 4]) << 4) | hex_digit(s[i + 5]));
                i += 6;
            }

            // Only attempt to fix sequences with a high byte (0x80+), which are the
            // ones that appear as the result of Facebook's broken encoding.
            if buf.len() > 1 || (buf.len() == 1 && buf[0] >= 0x80) {
                match std::str::from_utf8(&buf) {
                    Ok(decoded) => {
                        for ch in decoded.chars() {
                            let code = ch as u32;
                            if code > 0xFFFF {
                                // Encode as a JSON surrogate pair.
                                let adj = code - 0x10000;
                                result.push_str(&format!(
                                    "\\u{:04x}\\u{:04x}",
                                    0xD800 + (adj >> 10),
                                    0xDC00 + (adj & 0x3FF)
                                ));
                            } else if code > 0x7F {
                                result.push_str(&format!("\\u{:04x}", code));
                            } else {
                                result.push(ch);
                            }
                        }
                        continue;
                    }
                    Err(_) => {
                        // Not valid UTF-8 — preserve the original escape sequences.
                        result.push_str(&input[seq_start..i]);
                    }
                }
            } else {
                // Single-byte value or pure ASCII — preserve as-is.
                result.push_str(&input[seq_start..i]);
            }
        } else {
            // Facebook's DYI JSON export is ASCII (all non-ASCII is \uXXXX-escaped),
            // but browser-scraped JSON from JSON.stringify contains literal UTF-8 bytes.
            // Handle both: pass ASCII bytes through directly; decode multi-byte sequences.
            if s[i] < 0x80 {
                result.push(s[i] as char);
                i += 1;
            } else {
                let seq_len = if s[i] >= 0xF0 { 4 } else if s[i] >= 0xE0 { 3 } else { 2 };
                let end = (i + seq_len).min(len);
                if let Ok(s_str) = std::str::from_utf8(&s[i..end]) {
                    result.push_str(s_str);
                    i = end;
                } else {
                    result.push(s[i] as char);
                    i += 1;
                }
            }
        }
    }

    result
}

fn is_hex(b: u8) -> bool {
    matches!(b, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
}

fn hex_digit(b: u8) -> u8 {
    if b <= b'9' {
        b - b'0'
    } else if b <= b'F' {
        b - b'A' + 10
    } else {
        b - b'a' + 10
    }
}
