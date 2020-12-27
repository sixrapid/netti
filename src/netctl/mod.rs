mod parse;

use std::{fs, io, str::FromStr};

use self::parse::parse_netctl_profile;

#[derive(Debug, PartialEq)]
pub enum Connection {
    Wired,
    Wireless,
    None,
}

impl Default for Connection {
    fn default() -> Self {Connection::None}
}

// this can not return an err
impl FromStr for Connection {
    type Err = ();
    fn from_str(input: &str) -> Result<Connection, Self::Err> {
        match input {
            "wired" => Ok(Connection::Wired),
            "wireless" => Ok(Connection::Wireless),
            _ => Ok(Connection::None),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Status {
    Active,
    Enabled,
    Disabled,
    None,
}

impl Default for Status {
    fn default() -> Self {Status::None}
}

#[derive(Default, Debug)]
pub struct Profile {
    pub connection: Connection,
    pub interface: String,
    pub essid: String,
    pub status: Status,
    pub description: String,
}


impl Profile {
    pub fn new() -> Profile {
        Profile::default()
    }
}

pub fn get_profiles() -> Result<Vec<Profile>, io::Error> {
    let profiles: Vec<Profile> = fs::read_dir("/etc/netctl")?
        .filter_map(|res| res.ok())
        .filter_map(|de| fs::read_to_string(de.path()).ok())
        .filter_map(|s| parse_netctl_profile(&s))
        .collect();

    Ok(profiles)
}