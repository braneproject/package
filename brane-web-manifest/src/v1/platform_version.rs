use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformVersionResource {
    min_code: u32,
    target_code: u32,
    release_type: String,
}
