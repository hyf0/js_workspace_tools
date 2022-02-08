use std::path::Path;

use crate::{
    get_workspace_infos, search_package_json_files, structs::workspace_info::WorkspaceInfos,
};
use lets_find_up::FindUpOptions;
use log::debug;
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

pub fn get_pnpm_wrokspaces(cwd: &str) -> WorkspaceInfos {
    let pnpm_workspaces_root = get_pnpm_workspace_root(cwd);
    debug!("pnpm: pnpm_workspaces_root {:?}", pnpm_workspaces_root);
    let pnpm_workspaces_file = path::join!(&pnpm_workspaces_root, "pnpm-workspace.yaml");
    debug!("pnpm: pnpm_workspaces_file {:?}", pnpm_workspaces_file);
    let yaml = std::fs::read_to_string(pnpm_workspaces_file).unwrap();

    let pnpm_workspaces: PnpmWorkspaces = serde_yaml::from_str(&yaml).unwrap();
    debug!("pnpm: pnpm_workspaces {:?}", pnpm_workspaces);

    let package_paths = search_package_json_files(&pnpm_workspaces_root, &pnpm_workspaces.packages);
    debug!("pnpm: package_paths {:?}", package_paths);
    let workspace_infos = get_workspace_infos(&package_paths);
    debug!("pnpm: workspace_info {:?}", workspace_infos);

    return workspace_infos;
}
