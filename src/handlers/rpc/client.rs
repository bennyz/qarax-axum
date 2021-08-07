use super::*;

use node::{node_client::NodeClient, Response as NodeResponse, VmConfig, VmId};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Client {
    client: Arc<RwLock<NodeClient<tonic::transport::Channel>>>,
}
