use std::path::PathBuf;

/**
 * Starting from `cwd`, searches up the directory hierarchy for `pathName`
 */
pub fn search_up(path_name: &str, cwd: &str) -> Option<String> {
    let root = path::parse(&cwd).root;
    let mut cwd = cwd.to_string();

    let mut found = false;

    while !found && cwd.as_str() != root {
        if PathBuf::from(path::join!(&cwd, path_name)).exists() {
            found = true;
            break;
        }

        cwd = path::dirname(&cwd);
    }

    if found {
        Some(cwd.to_string())
    } else {
        None
    }
}

pub fn find_git_root(cwd: &str) -> Option<String> {
    search_up(".git", cwd)
}

pub fn find_package_root(cwd: &str) -> Option<String> {
    search_up("package.json", cwd)
}

pub fn get_change_path(cwd: &str) -> Option<String> {
    find_git_root(cwd).map(|git_root| path::join!(&git_root, "change"))
}

pub fn is_child_of(child: &str, parent: &str) -> bool {
    let relative_path = path::relative(child, parent);
    regex::Regex::new("^[./\\]+$")
        .unwrap()
        .is_match(&relative_path)
}
