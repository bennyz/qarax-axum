use super::BootSource;
use super::Drive;
use super::Logger;
use super::MachineConfiguration;
use super::NetworkInterface;

use crate::http::client::{Method, VmmClient};

use anyhow::Result;

#[derive(Serialize, Debug)]
pub struct Machine {
    #[serde(skip_serializing)]
    pub vm_id: String,

    #[serde(skip_serializing)]
    client: VmmClient,

    #[serde(rename(serialize = "machine-config"))]
    machine_configuration: MachineConfiguration,

    #[serde(rename(serialize = "boot-source"))]
    boot_source: BootSource,
    drives: Vec<Drive>,

    #[serde(rename(serialize = "network-interfaces"))]
    pub network_interfaces: Vec<NetworkInterface>,
    logger: Logger,

    #[serde(skip_serializing)]
    pid: Option<u32>,
}

impl Machine {
    pub fn new(
        vm_id: String,
        socket_path: String,
        machine_configuration: MachineConfiguration,
        boot_source: BootSource,
        drives: Vec<Drive>,
        network_interfaces: Vec<NetworkInterface>,
        logger: Logger,
        pid: Option<u32>,
    ) -> Self {
        Machine {
            vm_id,
            client: VmmClient::new(socket_path),
            machine_configuration,
            boot_source,
            drives,
            network_interfaces,
            logger,
            pid,
        }
    }

    pub async fn configure_boot_source(&self) -> Result<String> {
        todo!()
    }

    pub async fn configure_drive(&self) -> Result<String> {
        todo!()
    }

    pub async fn configure_logger(&self) -> Result<String> {
        todo!()
    }


    pub async fn start(&self) -> Result<String> {
       todo!()
    }

    pub async fn stop(&mut self) -> Result<()> {
        todo!()
    }
}