use std::env::VarError;


#[derive(Clone, Copy)]
pub enum WorkspaceManager {
    Yarn,
    Pnpm,
    Rush,
    Npm,
    Lerna,
}

impl WorkspaceManager {
    pub fn get_preferred_from_env() -> Result<WorkspaceManager, VarError> {
        std::env::var("PREFERRED_WORKSPACE_MANAGER")
            .map(|preferred| {
                let clinet = match preferred.as_str() {
                    "yarn" => WorkspaceManager::Yarn,
                    "pnpm" => WorkspaceManager::Pnpm,
                    "rush" => WorkspaceManager::Rush,
                    "npm" => WorkspaceManager::Npm,
                    "lerna" => WorkspaceManager::Lerna,
                    _ => panic!("Not supported manager: {}", preferred),
                };
                clinet
            })
    }
}


