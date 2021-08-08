use super::*;

use node::{node_client::NodeClient, Response as NodeResponse, VmConfig, VmId};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tonic::{IntoRequest, Request};

#[derive(Clone, Debug)]
pub struct Client {
    client: Arc<RwLock<NodeClient<tonic::transport::Channel>>>,
}

impl Client {
    pub async fn connect(addr: SocketAddr) -> Result<Client, tonic::transport::Error> {
        let client = NodeClient::connect(format!("http://{}:{}", addr.ip(), addr.port())).await?;

        Ok(Self {
            client: Arc::new(RwLock::new(client)),
        })
    }

    pub async fn health_check(
        self,
        request: impl IntoRequest<()>,
    ) -> Result<tonic::Response<(NodeResponse)>, tonic::Status> {
        Ok(self.client.write().await.health_check(request).await?)
    }
}
