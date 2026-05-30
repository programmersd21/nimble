/// Package registry protocol for Nimble
/// 
/// A registry is a Git repository containing a `registry.toml` file
/// that lists available packages.
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RegistryManifest {
    pub name: String,
    pub description: String,
    pub packages: HashMap<String, PackageEntry>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PackageEntry {
    pub description: String,
    pub repository: String,
    pub version: String,
    pub license: Option<String>,
    pub authors: Vec<String>,
    pub keywords: Vec<String>,
}

impl RegistryManifest {
    /// Parse from TOML string
    pub fn from_toml(input: &str) -> Result<Self, String> {
        toml::from_str(input).map_err(|e| format!("invalid registry manifest: {}", e))
    }

    /// Look up a package by name
    pub fn find_package(&self, name: &str, version: Option<&str>) -> Option<&PackageEntry> {
        self.packages.get(name).and_then(|pkg| {
            if let Some(ver) = version {
                if pkg.version == ver {
                    Some(pkg)
                } else {
                    None
                }
            } else {
                Some(pkg)
            }
        })
    }
}

/// Default Nimble registry URL
pub const DEFAULT_REGISTRY: &str = "https://github.com/nimble-lang/registry";

/// Search packages from a registry
pub fn search_registry(registry_url: &str, query: &str) -> Result<Vec<(String, PackageEntry)>, String> {
    // Clone/fetch the registry repo
    // Parse the registry.toml
    // Search for packages matching the query
    // Return results
    Err("registry not yet implemented".to_string())
}
