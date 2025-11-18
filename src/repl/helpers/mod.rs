
use std::path::Path;
use std::net::IpAddr;

use animusd_lib::{
    protocol::{ AnimusAction, AnimusCommand },
    error::{ CommandError, ConfigError }
};


// Check if a proposed animus name fits the formatting requirements. 
// (a-Z, 0-9, and underscores)
pub(crate) fn valid_animus_name(name: &str) -> bool {
    // Should unwrap a valid regular expression.
    let valid_name = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    valid_name.is_match(name)
}

