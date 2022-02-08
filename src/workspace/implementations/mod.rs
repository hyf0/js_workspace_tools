pub mod pnpm;
pub use pnpm::*;
pub mod yarn;

use std::path::Path;

use lets_find_up::FindUpOptions;

use super::workspace_manager::WorkspaceManager;

static SUPPORTED_MANANGERS: [(&str, WorkspaceManager); 5] = [
    ("lerna.json", WorkspaceManager::Lerna),
    ("yarn.lock", WorkspaceManager::Lerna),
    ("pnpm-workspace.yaml", WorkspaceManager::Lerna),
    ("rush.json", WorkspaceManager::Lerna),
    ("package-lock.json", WorkspaceManager::Lerna),
];

pub fn get_workspace_implementation(cwd: &str) -> Option<WorkspaceManager> {
    SUPPORTED_MANANGERS
        .iter()
        .map(|(lockfile_name, manager)| {
            lets_find_up::find_up_with(
                lockfile_name,
                FindUpOptions {
                    cwd: &Path::new(cwd),
                    ..Default::default()
                },
            )
            .unwrap()
            .map(|_| *manager)
        })
        .find(|s| s.is_some())
        .unwrap_or(None)
}
