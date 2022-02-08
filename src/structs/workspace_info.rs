use super::package_info::PackageInfo;

/// Workspaces is a generic term that refers to the set of features in the npm cli that provides support to managing multiple packages from your local files system from within a singular top-level, root package.
#[derive(Debug, Default)]
pub struct WorkspaceInfo {
  /// Refering to `package.json#name`
  pub name: String,
  pub path: String,
  pub package_json: PackageInfo,
}

pub type WorkspaceInfos = Vec<WorkspaceInfo>;