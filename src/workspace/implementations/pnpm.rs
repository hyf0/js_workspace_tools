use std::path::Path;

use crate::{
    get_workspace_infos, search_package_json_files, structs::workspace_info::WorkspaceInfo,
};
use lets_find_up::FindUpOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PnpmWorkspaces {
    pub packages: Vec<String>,
}

pub fn get_pnpm_workspace_root(cwd: &str) -> String {
    lets_find_up::find_up_with(
        "pnpm-workspace.yaml",
        FindUpOptions {
            cwd: &Path::new(cwd),
            ..Default::default()
        },
    )
    .expect("Could not find pnpm workspaces root")
    .and_then(|pnpm_workspaces_file| {
        pnpm_workspaces_file
            .parent()
            .map(|path| path.to_string_lossy().to_string())
    })
    .expect("Could not find pnpm workspaces root")
}

pub fn get_pnpm_wrokspaces(cwd: &str) -> WorkspaceInfo {
    let pnpm_workspaces_root = get_pnpm_workspace_root(cwd);
    let pnpm_workspaces_file = path::join!(&pnpm_workspaces_root, "pnpm-workspace.yaml");
    let yaml = std::fs::read_to_string(pnpm_workspaces_file).unwrap();

    let pnpm_workspaces: PnpmWorkspaces = serde_yaml::from_str(&yaml).expect("failed to parse pnpm-workspace.yaml");

    let package_paths = search_package_json_files(&pnpm_workspaces_root, &pnpm_workspaces.packages);
    let workspace_infos = get_workspace_infos(&package_paths);

    return workspace_infos;
}
