use crate::{get_workspaces, PackageInfo, PackageInfos};

/// Get paths to every package.json in workspaces, given a cwd
pub fn get_all_package_json_files(cwd: &str) -> Vec<String> {
    let workspaces = get_workspaces(&cwd);
    let package_json_files = workspaces
        .into_iter()
        .map(|workspace| workspace.package_json.__filename)
        .collect();
    package_json_files
}

/// Get every Parsed `package.json` in workspaces, given a cwd
pub fn get_package_infos(cwd: &str) -> PackageInfos {
    let package_json_files = get_all_package_json_files(cwd);
    package_json_files
        .into_iter()
        .map(|pkg_json_path| {
            let info = PackageInfo::from_path(pkg_json_path);
            (info.name.clone(), info)
        })
        .collect()
}


pub fn search_package_json_files<T: AsRef<str>>(workspaces_root: &str, glob_pats: &[T]) -> Vec<String> {
  glob_pats
      .into_iter()
      .map(|t| t.as_ref())
      .flat_map(|pat| {
          let pattern = path::join!(pat, "package.json").replace("\\", "/");
          // TODO: better perf
          globby::synced::globby_with(
              workspaces_root,
              &[pattern],
              &[
                  "**/node_modules/**".to_string(),
                  "**/.git".to_string(),
                  "**/.next".to_string(),
                  "**/.turbo".to_string(),
              ],
          )
          .into_iter()
          .map(|p| path::dirname(&p))
      })
      .map(|p| path::join!(&p))
      .collect()
}
