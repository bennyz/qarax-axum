extern crate firecracker_rust_sdk;

use crate::vmm_service::node::node_server::Node;
use crate::vmm_service::node::{
    Response as NodeResponse, Status as NodeStatus, VmConfig, VmId, VmList,
};
use tonic::{Code, Request, Response, Status};

pub(crate) mod node {
    tonic::include_proto!("node");
}

#[derive(Debug, Default)]
pub struct VmmService {}

#[tonic::async_trait]
impl Node for VmmService {
    async fn start_vm(&self, request: Request<VmConfig>) -> Result<Response<VmConfig>, Status> {
        unimplemented!()
    }

    async fn stop_vm(&self, request: Request<VmId>) -> Result<Response<NodeResponse>, Status> {
        unimplemented!()
    }

    async fn list_vms(&self, request: Request<()>) -> Result<Response<VmList>, Status> {
        unimplemented!()
    }
}
