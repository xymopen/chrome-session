use crate::serde_chromium_pickle::helpers;
use serde::de::{self, Deserialize as _};
use std::ffi::CString;
use std::os::raw::c_int;

pub type CommandID = u8;

#[derive(Debug)]
#[non_exhaustive]
// Identifier for commands written to file.
pub enum CommandKind {
    UpdateTabNavigation,
}

impl CommandKind {
    pub fn from(x: CommandID) -> Option<CommandKind> {
        match x {
            6 => Some(CommandKind::UpdateTabNavigation),
            _ => None,
        }
    }
}

// Various payload structures.

#[derive(serde::Deserialize, Debug)]
pub struct SerializedNavigationEntry {
    pub tab_id: c_int,
    pub index: c_int,
    #[serde(with = "helpers::cstring")]
    pub virtual_url: CString,
    #[serde(with = "helpers::string")]
    pub title: String,
    pub encoded_page_state_: Vec<u8>,
    pub transition_type: c_int,
    pub has_post_data: c_int,
    #[serde(with = "helpers::cstring")]
    pub referrer_url: CString,
    pub _deprecated_referrer_policy: c_int,
    #[serde(with = "helpers::cstring")]
    pub original_request_url: CString,
    pub is_overriding_user_agent: bool,
    pub timestamp: i64,
    pub _deprecated_search_terms: Vec<u16>,
    pub http_status_code: c_int,
    pub referrer_policy: c_int,
    #[serde(deserialize_with = "deserialize_pairs")]
    pub extended_info_map: Vec<(CString, CString)>,
}

pub fn deserialize_pairs<'de, D>(deserializer: D) -> Result<Vec<(CString, CString)>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let pairs: Result<Vec<_>, _> = Vec::<(Vec<u8>, Vec<u8>)>::deserialize(deserializer)?
        .into_iter()
        .map(|(key, value)| {
            CString::new(key).and_then(|key| CString::new(value).map(|value| (key, value)))
        })
        .collect();

    return pairs.or_else(|_| {
        Err(de::Error::custom(
            "an interior nul byte was found in the string",
        ))
    });
}
