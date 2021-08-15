/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 0.25.0
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// Vm : Defines the microVM running state. It is especially useful in the snapshotting context.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vm {
    #[serde(rename = "state")]
    pub state: State,
}

impl Vm {
    /// Defines the microVM running state. It is especially useful in the snapshotting context.
    pub fn new(state: State) -> Vm {
        Vm {
            state,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Paused")]
    Paused,
    #[serde(rename = "Resumed")]
    Resumed,
}

