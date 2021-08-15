/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 0.25.0
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// Metrics : Describes the configuration option for the metrics capability.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    /// Path to the named pipe or file where the JSON-formatted metrics are flushed.
    #[serde(rename = "metrics_path")]
    pub metrics_path: String,
}

impl Metrics {
    /// Describes the configuration option for the metrics capability.
    pub fn new(metrics_path: String) -> Metrics {
        Metrics {
            metrics_path,
        }
    }
}


