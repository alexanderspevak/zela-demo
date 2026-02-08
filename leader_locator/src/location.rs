use std::{
    fmt,
    net::{IpAddr, SocketAddr},
};
use zela_std::{RpcError, rpc_client::response::RpcContactInfo};

mod africa;
mod asia;
mod europe;
mod middle_east;
mod north_america;
mod oceania;
mod south_america;

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

                if matches_any(north_america::IP_ADRESSES) {
                    LeaderGeo::NorthAmerica
                } else if matches_any(europe::IP_ADRESSES) {
                    LeaderGeo::Europe
                } else if matches_any(asia::IP_ADRESSES) {
                    LeaderGeo::Asia
                } else if matches_any(south_america::IP_ADDRESSES) {
                    LeaderGeo::SouthAmerica
                } else if matches_any(africa::IP_ADDRESSES) {
                    LeaderGeo::Africa
                } else if matches_any(oceania::IP_ADDRESSES) {
                    LeaderGeo::Oceania
                } else if matches_any(middle_east::IP_ADDRESSES) {
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

pub fn get_geo_info(
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
