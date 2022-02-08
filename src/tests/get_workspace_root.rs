use crate::get_pnpm_workspace_root;



#[test]
fn handles_pnpm_workspace() {
  let repo_root = path::join!(&path::resolve!(), "fixtures/monorepo-pnpm");
  let workspace_root =  get_pnpm_workspace_root(&repo_root);
  assert_eq!(workspace_root, repo_root);
}