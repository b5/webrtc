#[cfg(test)]
mod network_type_test;

use crate::error::*;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::IpAddr;

pub(crate) const UDP: &str = "udp";
pub(crate) const TCP: &str = "tcp";
pub(crate) const QUIC: &str = "quic";

#[must_use]
pub fn supported_network_types() -> Vec<NetworkType> {
    vec![
        NetworkType::Udp4,
        NetworkType::Udp6,
        //NetworkType::TCP4,
        //NetworkType::TCP6,
        NetworkType::Quic4,
        NetworkType::Quic6,
    ]
}

/// Represents the type of network.
#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkType {
    #[serde(rename = "unspecified")]
    Unspecified,

    /// Indicates UDP over IPv4.
    #[serde(rename = "udp4")]
    Udp4,

    /// Indicates UDP over IPv6.
    #[serde(rename = "udp6")]
    Udp6,

    /// Indicates TCP over IPv4.
    #[serde(rename = "tcp4")]
    Tcp4,

    /// Indicates TCP over IPv6.
    #[serde(rename = "tcp6")]
    Tcp6,

    /// Indicates TCP over IPv4.
    #[serde(rename = "quic4")]
    Quic4,

    /// Indicates TCP over IPv6.
    #[serde(rename = "quic6")]
    Quic6,
}

impl From<u8> for NetworkType {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Udp4,
            2 => Self::Udp6,
            3 => Self::Tcp4,
            4 => Self::Tcp6,
            5 => Self::Quic4,
            6 => Self::Quic6,
            _ => Self::Unspecified,
        }
    }
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Self::Udp4 => "udp4",
            Self::Udp6 => "udp6",
            Self::Tcp4 => "tcp4",
            Self::Tcp6 => "tcp6",
            Self::Quic4 => "quic4",
            Self::Quic6 => "quic6",
            Self::Unspecified => "unspecified",
        };
        write!(f, "{}", s)
    }
}

impl Default for NetworkType {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl NetworkType {
    /// Returns true when network is UDP4 or UDP6.
    #[must_use]
    pub fn is_udp(self) -> bool {
        self == Self::Udp4 || self == Self::Udp6
    }

    /// Returns true when network is TCP4 or TCP6.
    #[must_use]
    pub fn is_tcp(self) -> bool {
        self == Self::Tcp4 || self == Self::Tcp6
    }

    #[must_use]
    pub fn is_quic(self) -> bool {
        self == Self::Quic4 || self == Self::Quic6
    }

    /// Returns the short network description.
    #[must_use]
    pub fn network_short(self) -> String {
        match self {
            Self::Udp4 | Self::Udp6 => UDP.to_owned(),
            Self::Tcp4 | Self::Tcp6 => TCP.to_owned(),
            Self::Quic4 | Self::Quic6 => QUIC.to_owned(),
            Self::Unspecified => "Unspecified".to_owned(),
        }
    }

    /// Returns true if the network is reliable.
    #[must_use]
    pub const fn is_reliable(self) -> bool {
        match self {
            Self::Tcp4 | Self::Tcp6 => true,
            Self::Udp4 | Self::Udp6 | Self::Unspecified => false,
            // TODO(b5): is this true? pretty sure quic is reliable
            Self::Quic4 | Self::Quic6 => true,
        }
    }

    /// Returns whether the network type is IPv4 or not.
    #[must_use]
    pub const fn is_ipv4(self) -> bool {
        match self {
            Self::Udp4 | Self::Tcp4 | Self::Quic4 => true,
            Self::Udp6 | Self::Tcp6 | Self::Quic6 | Self::Unspecified => false,
        }
    }

    /// Returns whether the network type is IPv6 or not.
    #[must_use]
    pub const fn is_ipv6(self) -> bool {
        match self {
            Self::Udp6 | Self::Tcp6 | Self::Quic6 => true,
            Self::Udp4 | Self::Tcp4 | Self::Quic4 | Self::Unspecified => false,
        }
    }
}

/// Determines the type of network based on the short network string and an IP address.
pub(crate) fn determine_network_type(network: &str, ip: &IpAddr) -> Result<NetworkType> {
    let ipv4 = ip.is_ipv4();
    let net = network.to_lowercase();
    if net.starts_with(UDP) {
        if ipv4 {
            Ok(NetworkType::Udp4)
        } else {
            Ok(NetworkType::Udp6)
        }
    } else if net.starts_with(TCP) {
        if ipv4 {
            Ok(NetworkType::Tcp4)
        } else {
            Ok(NetworkType::Tcp6)
        }
    } else if net.starts_with(QUIC) {
        if ipv4 {
            Ok(NetworkType::Quic4)
        } else {
            Ok(NetworkType::Quic6)
        }
    } else {
        Err(Error::ErrDetermineNetworkType)
    }
}
