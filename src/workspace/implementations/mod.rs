pub mod pnpm;
pub use pnpm::*;
pub mod yarn;

use std::path::Path;

use lets_find_up::FindUpOptions;

use super::workspace_manager::WorkspaceManager;

pub fn get_workspace_implementation(cwd: &str) -> Option<WorkspaceManager> {
    lets_find_up::find_up_with(
        "pnpm-workspace.yaml",
        FindUpOptions {
            cwd: &Path::new(cwd),
            ..Default::default()
        },
    )
    .unwrap()
    .map(|_| WorkspaceManager::Pnpm)
}
