use derivative::Derivative;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Ord, PartialOrd, Derivative)]
#[derivative(Debug)]
pub enum Vendor {
    AMD,
    Intel,
    NVIDIA,

    #[derivative(Debug = "transparent")]
    Other(String),
}

impl Vendor {
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "amd" => Self::AMD,
            "nvidia" => Self::NVIDIA,
            "intel" => Self::Intel,

            _ => Self::Other(s.to_string()),
        }
    }

    pub fn normalize_device<'d>(&self, device: &'d str) -> Cow<'d, str> {
        match self {
            Self::AMD => {
                lazy_static! {
                    static ref AMD_RE: Regex = Regex::new(r#"(AMD\s*)|(\([^)]*\)+)"#).unwrap();
                }

                AMD_RE.replace_all(device, "")
            },
            Self::NVIDIA => {
                lazy_static! {
                    static ref NVIDIA_RE: Regex =
                        Regex::new(r#"(NVIDIA\s*)|((?i)GeForce\s*)"#).unwrap();
                }

                NVIDIA_RE.replace_all(device, "")
            },
            Self::Intel => {
                lazy_static! {
                    static ref INTEL_RE: Regex = Regex::new(r#"(Intel\s*)?(\(R\)\s*)?"#).unwrap();
                }

                INTEL_RE.replace_all(device, "")
            },

            _ => Cow::Borrowed(device),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Ord, PartialOrd, Debug)]
pub struct VendorDevice {
    pub vendor: Vendor,
    pub device: String,
}

impl VendorDevice {
    pub fn parse(vendor: &str, device: &str) -> Self {
        let vendor = Vendor::parse(vendor);
        let device = vendor.normalize_device(device).to_string();

        Self { vendor, device }
    }
}

impl Display for VendorDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.vendor, self.device)
    }
}
