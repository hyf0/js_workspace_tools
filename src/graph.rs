use crate::{get_internal_deps, structs::package_info::PackageInfos};

/// edges of dependency/devDependency => package
pub fn get_package_graph(pkgs: &PackageInfos) -> Vec<(&str, &str)> {
    let mut edges = vec![];

    pkgs.iter().for_each(|(pkg, info)| {
        let deps = get_internal_deps(info, pkgs);
        deps.into_iter().for_each(|dep| {
            edges.push((dep, pkg.as_str()));
        });
    });

    edges
}
