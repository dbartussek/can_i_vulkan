use crate::{vendor_device::Vendor, OsCategory, VendorDevice};
use eyre::eyre;
use reqwest::Url;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReportSummary {
    #[serde(rename = "vendorid")]
    pub vendor_id: String,
    #[serde(rename = "deviceid")]
    pub device_id: String,
    #[serde(rename = "vendorname")]
    pub vendor_name: String,
    #[serde(rename = "devicename")]
    pub device_name: String,
    #[serde(rename = "devicetype")]
    pub device_type: String,
    #[serde(rename = "apiversion")]
    pub api_version: String,
    #[serde(rename = "driverversionraw")]
    pub driver_version_raw: String,
    #[serde(rename = "driverversion")]
    pub driver_version: String,
    #[serde(rename = "osname")]
    pub os_name: String,
    #[serde(rename = "osversion")]
    pub os_version: String,
    #[serde(rename = "osarchitecture")]
    pub os_architecture: String,
    #[serde(rename = "headerversion")]
    pub header_version: Option<String>,
    #[serde(rename = "reportversion")]
    pub report_version: String,

    pub url: String,
}

impl ReportSummary {
    pub fn id(&self) -> eyre::Result<String> {
        let url = Url::parse(&self.url)?;
        url.query_pairs()
            .find(|(key, _)| key == "id")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| eyre!("url has no id"))
    }

    pub fn os_category(&self) -> OsCategory {
        OsCategory::parse(&self.os_name)
    }

    pub fn api_semver(&self) -> eyre::Result<Version> {
        Ok(Version::parse(&self.api_version)?)
    }

    pub fn driver_semver(&self) -> eyre::Result<Version> {
        Ok(Version::parse(&self.driver_version)?)
    }

    pub fn os_semver(&self) -> eyre::Result<Version> {
        Ok(Version::parse(&self.os_version)?)
    }

    pub fn vendor(&self) -> Vendor {
        Vendor::parse(&self.vendor_name)
    }

    pub fn vendor_device(&self) -> VendorDevice {
        VendorDevice::parse(&self.vendor_name, &self.device_name)
    }
}
