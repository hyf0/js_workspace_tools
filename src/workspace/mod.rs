pub mod implementations;
pub mod package;
pub mod workspace_manager;

pub use implementations::*;
pub use package::*;
pub use workspace_manager::*;

use crate::{PackageInfo, WorkspaceInfo, WorkspaceInfos};

pub fn get_workspaces(cwd: &str) -> WorkspaceInfos {
    let preferred = WorkspaceManager::get_preferred_from_env()
        .unwrap_or_else(|_| get_workspace_implementation(cwd).unwrap());
    match preferred {
        WorkspaceManager::Pnpm => get_pnpm_wrokspaces(cwd),
        _ => vec![],
    }
}

pub fn find_workspace_path<'a>(workspaces: &'a [WorkspaceInfo], pkg_name: &str) -> Option<&'a str> {
    workspaces
        .into_iter()
        .find(|info| info.name.as_str() == pkg_name)
        .map(|info| info.path.as_ref())
}

pub fn get_workspace_root(cwd: &str) -> Option<String> {
    let preferred = WorkspaceManager::get_preferred_from_env()
        .unwrap_or_else(|_| get_workspace_implementation(cwd).unwrap());
    match preferred {
        WorkspaceManager::Pnpm => Some(get_pnpm_workspace_root(cwd)),
        _ => None,
    }
}

pub fn get_workspace_infos<T: AsRef<str>>(workspace_paths: &[T]) -> WorkspaceInfos {
    workspace_paths
        .into_iter()
        .map(|s| s.as_ref())
        .map(|workspace_path| {
            let pkg_json_path = path::join!(workspace_path, "package.json");
            let pkg_info = PackageInfo::from_path(pkg_json_path);
            
            WorkspaceInfo {
                name: pkg_info.name.clone(),
                path: workspace_path.to_string(),
                package_json: pkg_info,
            }
        })
        .collect()
}
