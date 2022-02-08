use super::package_info::PackageInfo;

/// Workspaces is a generic term that refers to the set of features in the npm cli that provides support to managing multiple packages from your local files system from within a singular top-level, root package.
#[derive(Debug, Default)]
pub struct WorkspaceItem {
  /// Refering to `package.json#name`
  pub name: String,
  /// dirname of contained `package.json`
  pub path: String,
  pub package_json: PackageInfo,
}

pub type WorkspaceInfo = Vec<WorkspaceItem>;