use std::{
    fmt,
    net::{IpAddr, SocketAddr},
};
use zela_std::{RpcError, rpc_client::response::RpcContactInfo};

const NORTH_AMERICA_PREFIXES: &[&str] = &[
    "64.130.", "208.115.", "198.13.", "44.", "204.16.", "15.204.", "40.160.", "23.109.", "74.118.",
    "67.209.", "199.48.", "199.127.", "107.6.", "72.251.", "104.238.", "104.204.", "18.208.",
    "34.232.", "50.150.", "108.171.", "63.251.", "38.46.", "47.76.", "132.145.", "35.197.",
];
const EUROPE_PREFIXES: &[&str] = &[
    "135.125.", "5.199.", "79.137.", "162.55.", "65.21.", "188.42.", "145.239.", "54.220.",
    "91.242.", "109.94.", "31.40.", "212.237.", "82.27.", "141.98.", "185.189.", "88.216.",
    "57.129.", "95.214.", "212.69.",
];
const ASIA_PREFIXES: &[&str] = &[
    "61.111.", "160.202.", "60.244.", "122.116.", "103.106.", "43.130.", "8.211.",
];
const SOUTH_AMERICA_PREFIXES: &[&str] = &["186.233."];
const AFRICA_PREFIXES: &[&str] = &[];
const OCEANIA_PREFIXES: &[&str] = &[];
const MIDDLE_EAST_PREFIXES: &[&str] = &[];

pub enum LeaderGeo {
    NorthAmerica,
    SouthAmerica,
    Oceania,
    MiddleEast,
    Africa,
    Asia,
    Europe,
    Unknown,
}

impl From<&IpAddr> for LeaderGeo {
    fn from(ip: &IpAddr) -> Self {
        match ip {
            IpAddr::V6(addr) => {
                log::warn!("IPv6 address detected: {}", addr);
                LeaderGeo::Unknown
            }

            IpAddr::V4(addr) => {
                let ip_str = addr.to_string();

                let matches_any = |prefixes: &[&str]| -> bool {
                    prefixes.iter().any(|&prefix| ip_str.starts_with(prefix))
                };

                if matches_any(NORTH_AMERICA_PREFIXES) {
                    LeaderGeo::NorthAmerica
                } else if matches_any(EUROPE_PREFIXES) {
                    LeaderGeo::Europe
                } else if matches_any(ASIA_PREFIXES) {
                    LeaderGeo::Asia
                } else if matches_any(SOUTH_AMERICA_PREFIXES) {
                    LeaderGeo::SouthAmerica
                } else if matches_any(AFRICA_PREFIXES) {
                    LeaderGeo::Africa
                } else if matches_any(OCEANIA_PREFIXES) {
                    LeaderGeo::Oceania
                } else if matches_any(MIDDLE_EAST_PREFIXES) {
                    LeaderGeo::MiddleEast
                } else {
                    log::warn!("IPv4 address with unknown prefix detected: {}", ip_str);
                    LeaderGeo::Unknown
                }
            }
        }
    }
}

impl fmt::Display for LeaderGeo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeaderGeo::NorthAmerica => write!(f, "North America"),
            LeaderGeo::SouthAmerica => write!(f, "South America"),
            LeaderGeo::Oceania => write!(f, "Oceania"),
            LeaderGeo::MiddleEast => write!(f, "Middle East"),
            LeaderGeo::Asia => write!(f, "Asia"),
            LeaderGeo::Africa => write!(f, "Asia"),
            LeaderGeo::Europe => write!(f, "Europe"),
            LeaderGeo::Unknown => write!(f, "Unknown"),
        }
    }
}

pub enum ZelaServerRegion {
    Frankfurt,
    Dubai,
    NewYork,
    Tokyo,
}

impl fmt::Display for ZelaServerRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZelaServerRegion::Frankfurt => write!(f, "Berlin"),
            ZelaServerRegion::Dubai => write!(f, "Dubai"),
            ZelaServerRegion::NewYork => write!(f, "New York"),
            ZelaServerRegion::Tokyo => write!(f, "Tokyo"),
        }
    }
}

impl From<&LeaderGeo> for ZelaServerRegion {
    fn from(leader_geo: &LeaderGeo) -> Self {
        match leader_geo {
            LeaderGeo::NorthAmerica => ZelaServerRegion::NewYork,
            LeaderGeo::SouthAmerica => ZelaServerRegion::NewYork,
            LeaderGeo::Europe => ZelaServerRegion::Frankfurt,
            LeaderGeo::Asia => ZelaServerRegion::Tokyo,
            LeaderGeo::Oceania => ZelaServerRegion::Tokyo,
            LeaderGeo::MiddleEast => ZelaServerRegion::Dubai,
            LeaderGeo::Africa => ZelaServerRegion::Dubai,
            LeaderGeo::Unknown => ZelaServerRegion::Dubai,
        }
    }
}

fn get_ip_from_contact_info(contact_info: &RpcContactInfo) -> Option<SocketAddr> {
    if let Some(addr) = contact_info.gossip {
        return Some(addr);
    }

    if let Some(addr) = contact_info.rpc {
        return Some(addr);
    }

    if let Some(addr) = contact_info.tvu {
        return Some(addr);
    }

    if let Some(addr) = contact_info.tpu {
        return Some(addr);
    }

    None
}

pub fn get_closest_zela_server_region(
    contact_info: &RpcContactInfo,
    slot: u64,
) -> Result<(ZelaServerRegion, LeaderGeo), RpcError<()>> {
    let ip = get_ip_from_contact_info(contact_info).ok_or(RpcError {
        code: 500,
        message: format!("Leader for slot: {} has no valid ip address", slot),
        data: None,
    })?;

    let leader_geo = LeaderGeo::from(&ip.ip());
    let zela_server_region = ZelaServerRegion::from(&leader_geo);

    Ok((zela_server_region, leader_geo))
}
