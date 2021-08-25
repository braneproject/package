use crate::v1::error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub app_id: String,
    pub name: String,
    pub short_name: String,
}

impl Manifest {
    pub fn from_str(content: &str) -> Result<Self, error::Error> {
        let manifest: Self = serde_json::from_str(content)?;
        Ok(manifest)
    }
}

#[test]
fn v1_manifest_full() {
    let json = r#"
      {
        "app_id": "test-app-id",
        "name": "My First Test App",
        "short_name": "Test App"
      }
    "#;

    let manifest = Manifest::from_str(&json);
    assert!(manifest.is_ok());
}

#[test]
fn v1_manifest_missing_app_id() {
    let json = r#"
      {
        "name": "My First Test App",
        "short_name": "Test App"
      }
    "#;

    let manifest = Manifest::from_str(&json);
    assert!(manifest.is_err());
}
