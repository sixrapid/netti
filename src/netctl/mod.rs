mod parse;

use std::{fmt, fs, io, path, str::FromStr};

use path::PathBuf;
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

// this struct holds the configuration options for any one netctl profiles.
// the configuration options are described in "man netctl.profile".
// the path-field should contain the path to the profile.
#[derive(Debug, Default)]
pub struct Profile {
    pub path: PathBuf,
    pub description: String,
    pub connection: Connection,
    pub interface: String,
    pub security: String,
    pub essid: String,
    pub status: Status,
}

#[derive(Debug, PartialEq)]
pub enum Connection {
    Ethernet,
    Wireless,
    Bond,
    Bridge,
    Dummy,
    Ppp,
    Pppoe,
    MobilePpp,
    OpenVswitch,
    Tunnel,
    Tuntap,
    Vlan,
    Macvlan,
    Wireguard,
}

impl Default for Connection {
    fn default() -> Self {Connection::Ethernet}
}

#[derive(Debug, Clone)]
pub struct ConnectionParseError;

impl fmt::Display for ConnectionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid connection type")
    }
}

impl FromStr for Connection {
    type Err = ConnectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethernet" => Ok(Connection::Ethernet),
            "wireless" => Ok(Connection::Wireless),
            "bond" => Ok(Connection::Bond),
            "bridge" => Ok(Connection::Bridge),
            "dummy" => Ok(Connection::Dummy),
            "ppp" => Ok(Connection::Ppp),
            "pppoe" => Ok(Connection::Pppoe),
            "mobile_ppp" => Ok(Connection::MobilePpp),
            "openvswitch" => Ok(Connection::OpenVswitch),
            "tunnel" => Ok(Connection::Tunnel),
            "tuntap" => Ok(Connection::Tuntap),
            "vlan" => Ok(Connection::Vlan),
            "macvlan" => Ok(Connection::Macvlan),
            "wireguard" => Ok(Connection::Wireguard),
            _ => Err(ConnectionParseError),
        }
    }
}

impl ToString for Connection {
    fn to_string(&self) -> String {
        let str = match self {
            Connection::Ethernet => "Ethernet",
            Connection::Wireless => "Wireless",
            Connection::Bond => "Bond",
            Connection::Bridge => "Bridge",
            Connection::Dummy => "Dummy",
            Connection::Ppp => "PPP",
            Connection::Pppoe => "PPPoE",
            Connection::MobilePpp => "Mobile PPP",
            Connection::OpenVswitch => "Open vSwitch",
            Connection::Tunnel => "Tunnel",
            Connection::Tuntap => "TUN/TAP",
            Connection::Vlan => "VLAN",
            Connection::Macvlan => "MACVLAN",
            Connection::Wireguard => "Wireguard",
        };

        str.to_string()
    }
}

impl Connection {
    // returns the name for the icon we want for this interface
    // e.g. Wireless returns "network-wireless"
    pub fn icon_name(&self) -> &str {
        match self {
            Connection::Ethernet => "network-wired-symbolic",
            Connection::Wireless => "network-wireless-symbolic",
            Connection::Bond => "network-bond-symbolic",
            Connection::Bridge => "network-bridge-symbolic",
            Connection::Dummy => "network-dummy-symbolic",
            Connection::Ppp => "network-ppp-symbolic",
            Connection::Pppoe => "network-ppp-symbolic",
            Connection::MobilePpp => "network-ppp-symbolic",
            Connection::OpenVswitch => "network-openvswitch-symbolic",
            Connection::Tunnel => "network-tunnel-symbolic",
            Connection::Tuntap => "network-tuntap-symbolic",
            Connection::Vlan => "network-vlan-symbolic",
            Connection::Macvlan => "network-macvlan-symbolic",
            Connection::Wireguard => "network-wireguard-symbolic",
        }
    }
}


impl Profile {
    pub fn new(path: PathBuf, connection: Connection, interface: String) -> Profile {
        Profile {
            path,
            connection,
            interface,
            ..Default::default()
        }
    }
}

// consider changing Option to Result
pub fn get_profile(direntry: fs::DirEntry) -> Option<Profile> {
    let path = direntry.path();
    let config = fs::read_to_string(&path).ok()?;

    let map = parse::parse_profile_to_hashmap(&config);

    // connection and interface are mandatory config options
    let connection = map.get("Connection")?.parse().ok()?;
    let interface = map.get("Interface")?.clone();

    let mut profile = Profile::new(path, connection, interface);

    // insert the other options
    profile.essid = map.get("ESSID").cloned().unwrap_or_default();
    
    Some(profile)
}

pub fn profile_iter() -> Result<impl Iterator<Item = Profile>, io::Error> {
    Ok(fs::read_dir("./test_netctl_folder/")?.filter_map(Result::ok).filter_map(get_profile))
}