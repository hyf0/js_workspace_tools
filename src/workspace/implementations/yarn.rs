// use crate::structs::workspace_info::WorkspaceInfos;

// pub fn get_yarn_workspace_root(cwd: String) -> String {
//   const yarnWorkspacesRoot = getPackageJsonWorkspaceRoot(cwd);

//   if (!yarnWorkspacesRoot) {
//     throw new Error("Could not find yarn workspaces root");
//   }

//   return yarnWorkspacesRoot;
// }

// pub fn getYarnWorkspaces(cwd: String) -> WorkspaceInfos {
//   const yarnWorkspacesRoot = getYarnWorkspaceRoot(cwd);
//   return getWorkspaceInfoFromWorkspaceRoot(yarnWorkspacesRoot);
// }
