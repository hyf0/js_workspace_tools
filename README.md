# js_workspace_tools

A collection of tools that are useful in a git-controlled monorepo that is managed by one of these software:

- [ ] lerna
- [ ] npm workspaces
- [ ] pnpm workspaces
- [ ] rush
- [ ] yarn workspaces

# Progress

- [x] export * from "./dependencies";
- [x] export * from "./getPackageInfos";
- [ ] export * from "./git";
- [x] export * from "./graph";
- [ ] export * from "./lockfile";
- [ ] export * from "./paths";
- [ ] export * from "./scope";
- [x] export * from "./types/PackageInfo";
- [x] export * from "./types/WorkspaceInfo";
- [x] export * from "./workspaces/findWorkspacePath";
- [x] export * from "./workspaces/getWorkspaces";
- [x] export * from "./workspaces/getWorkspaceRoot";
- [x] export * from "./workspaces/implementations/pnpm";
- [ ] export * from "./workspaces/implementations/rush";
- [ ] export * from "./workspaces/implementations/yarn";
- [ ] export * from "./workspaces/getChangedPackages";
- [ ] (Not goting to support this) export * from "./workspaces/listOfWorkspacePackageNames";
- [x] export * from "./workspaces/workspaces";

---

Most functions are porting from [workspace-tools](https://github.com/microsoft/workspace-tools). Wonderful work of [kenotron](https://github.com/kenotron).
