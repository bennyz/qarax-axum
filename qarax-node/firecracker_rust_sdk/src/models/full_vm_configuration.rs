/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 0.25.0
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullVmConfiguration {
    #[serde(rename = "balloon_device", skip_serializing_if = "Option::is_none")]
    pub balloon_device: Option<Box<crate::models::Balloon>>,
    /// Configurations for all block devices.
    #[serde(rename = "block_devices", skip_serializing_if = "Option::is_none")]
    pub block_devices: Option<Vec<crate::models::Drive>>,
    #[serde(rename = "boot_source", skip_serializing_if = "Option::is_none")]
    pub boot_source: Option<Box<crate::models::BootSource>>,
    #[serde(rename = "logger", skip_serializing_if = "Option::is_none")]
    pub logger: Option<Box<crate::models::Logger>>,
    #[serde(rename = "machine_config", skip_serializing_if = "Option::is_none")]
    pub machine_config: Option<Box<crate::models::MachineConfiguration>>,
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Box<crate::models::Metrics>>,
    #[serde(rename = "mmds_config", skip_serializing_if = "Option::is_none")]
    pub mmds_config: Option<Box<crate::models::MmdsConfig>>,
    /// Configurations for all net devices.
    #[serde(rename = "net_devices", skip_serializing_if = "Option::is_none")]
    pub net_devices: Option<Vec<crate::models::NetworkInterface>>,
    #[serde(rename = "vsock_device", skip_serializing_if = "Option::is_none")]
    pub vsock_device: Option<Box<crate::models::Vsock>>,
}

impl FullVmConfiguration {
    pub fn new() -> FullVmConfiguration {
        FullVmConfiguration {
            balloon_device: None,
            block_devices: None,
            boot_source: None,
            logger: None,
            machine_config: None,
            metrics: None,
            mmds_config: None,
            net_devices: None,
            vsock_device: None,
        }
    }
}
