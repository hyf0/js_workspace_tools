use std::{collections::HashMap};

/// Parsed `package.json`
#[derive(Debug, Default)]
pub struct PackageInfo {
    pub name: String,
    /// The `__filename` represents the filename of the package.json being parsed.
    pub __filename: String,
    pub version: String,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub peer_dependencies: HashMap<String, String>,
    pub private: Option<bool>,
    pub group: Option<String>,
    pub scripts: HashMap<String, String>,
    /// Original Value of `package.json` being parsed by serde_json
    pub __origin: serde_json::Value,
}

impl PackageInfo {
    pub fn from_json_value(value: serde_json::Value, package_json_path: String) -> Self {
        let dependencies: HashMap<String, String> = value
            .get("dependencies")
            .map_or(Default::default(), |value| {
                serde_json::from_value(value.clone()).unwrap()
            });
        let dev_dependencies: HashMap<String, String> = value
            .get("devDependencies")
            .map_or(Default::default(), |value| {
                serde_json::from_value(value.clone()).unwrap()
            });
        let peer_dependencies: HashMap<String, String> = value
            .get("peerDependencies")
            .map_or(Default::default(), |value| {
                serde_json::from_value(value.clone()).unwrap()
            });
        let scripts: HashMap<String, String> =
            value.get("scripts").map_or(Default::default(), |value| {
                serde_json::from_value(value.clone()).unwrap()
            });

        Self {
            name: value.get("name").unwrap().as_str().unwrap().to_string(),
            __filename: package_json_path,
            version: value.get("version").unwrap().as_str().unwrap().to_string(),
            dependencies,
            dev_dependencies,
            peer_dependencies,
            private: Default::default(),
            group: Default::default(),
            scripts,
            __origin: value,
            // package_json_path:
        }
    }

    pub fn from_path(pkg_json_path: String) -> Self {
        let v = serde_json::from_str(&std::fs::read_to_string(&pkg_json_path).unwrap())
            .map(|s| PackageInfo::from_json_value(s, pkg_json_path))
            .unwrap();
        v
    }
}

pub type PackageInfos = HashMap<String, PackageInfo>;
