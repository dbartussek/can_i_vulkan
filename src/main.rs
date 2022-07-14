pub mod get;
pub mod iterator;
pub mod os_category;
pub mod report_summary;
pub mod vendor_device;

use crate::{
    get::{api_get, CACHE_PATH},
    iterator::{count_key, group_by},
    os_category::OsCategory,
    report_summary::ReportSummary,
    vendor_device::VendorDevice,
};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = std::fs::create_dir_all(CACHE_PATH);

    let query_devices: HashMap<String, HashMap<String, f32>> =
        ron::from_str(&std::fs::read_to_string("query_devices.ron")?)?;
    let query_devices: BTreeMap<VendorDevice, f32> = query_devices
        .iter()
        .map(|(vendor, devices)| {
            devices.iter().map(|(device, market_share)| {
                (
                    VendorDevice::parse(vendor.as_str(), device.as_str()),
                    *market_share,
                )
            })
        })
        .flatten()
        .collect();

    let report_list = api_get(
        "https://vulkan.gpuinfo.org/api/",
        "v2/getreportlist.php",
        Default::default(),
    )
    .await?;
    let report_list: Vec<ReportSummary> = serde_json::from_str(&report_list)?;
    let x86_64 = report_list
        .iter()
        .filter(|r| r.os_architecture == "x86_64")
        .collect_vec();
    let windows = x86_64
        .iter()
        .copied()
        .filter(|r| r.os_category() == OsCategory::Windows)
        .collect_vec();

    let os = count_key(report_list.iter().map(|r| r.os_category()));
    println!("{:#?}", os);

    println!("{:#?}", count_key(windows.iter().map(|r| r.vendor())));

    let by_device = group_by(windows.iter().copied(), |report| report.vendor_device());

    println!();

    for (query, _) in query_devices.iter() {
        match by_device.get(query) {
            None => println!("{:?} not found", query),
            Some(reports) => println!(
                "{:?}: {}",
                query,
                reports
                    .iter()
                    .map(|report| report.api_semver().unwrap())
                    .max()
                    .unwrap()
            ),
        }
    }

    Ok(())
}
