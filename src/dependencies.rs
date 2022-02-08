use std::collections::{HashMap, HashSet, VecDeque};

use crate::structs::package_info::{PackageInfo, PackageInfos};

static TRANSPARNT: [&str; 0] = [];

/// A graph about dependency to package.
/// - `(react, app)` means package app's dependencies/dev_dependencies contains package react
/// - `(None, app)` means package app has no dependencies/dev_dependencies
#[inline]
fn get_package_graph(pkgs: &PackageInfos) -> Vec<(Option<&str>, &str)> {
    // let transparnt: Vec<&str> = Default::default();
    get_package_graph_with_scope(pkgs, &TRANSPARNT)
}

/// A graph about dependency to package.
/// - `(react, app)` means package app's dependencies/dev_dependencies contains package react
/// - `(None, app)` means package app has no dependencies/dev_dependencies
fn get_package_graph_with_scope<'a, T: AsRef<str>>(
    pkgs: &'a PackageInfos,
    scope: &'a [T],
) -> Vec<(Option<&'a str>, &'a str)> {
    let mut edges = vec![];

    let mut visited = HashSet::new();
    let mut stack: Vec<&str> = if scope.len() > 0 {
        scope.into_iter().map(|s| s.as_ref()).collect()
    } else {
        pkgs.keys().map(|s| s.as_str()).collect()
    };

    while let Some(pkg) = stack.pop() {
        if visited.contains(pkg) {
            continue;
        }

        visited.insert(pkg);
        let info = pkgs.get(pkg).unwrap();
        let deps = get_internal_deps(info, &pkgs);
        if deps.len() > 0 {
            deps.into_iter().for_each(|dep| {
                stack.push(dep);
                edges.push((Some(dep), pkg));
            });
        } else {
            edges.push((None, pkg));
        }
    }

    edges
}

/// HashMap<PackageName, HashSet<Package's dependcies/devDependcies>>
pub fn get_dependent_map(pkgs: &PackageInfos) -> HashMap<&str, HashSet<&str>> {
    let graph = get_package_graph(pkgs);
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    graph.into_iter().for_each(|(from, to)| {
        if !map.contains_key(&to) {
            map.insert(to, Default::default());
        }

        if let Some(from) = from {
            map.get_mut(&to).unwrap().insert(from);
        }
    });

    map
}
/// for a package graph of a->b->c (where b depends on a), transitive consumers of a are b & c and their consumers (or what are the consequences of a)
#[inline]
pub fn get_transitive_consumers<'a, T: AsRef<str>>(
    targets: &'a [T],
    pkgs: &'a PackageInfos,
) -> Vec<&'a str> {
    get_transitive_consumers_with_scope::<T, &str>(targets, pkgs, &[])
}

/// for a package graph of a->b->c (where b depends on a), transitive consumers of a are b & c and their consumers (or what are the consequences of a)
pub fn get_transitive_consumers_with_scope<'a, T: AsRef<str>, U: AsRef<str>>(
    targets: &'a [T],
    pkgs: &'a PackageInfos,
    scope: &'a [U],
) -> Vec<&'a str> {
    let graph = get_package_graph_with_scope(pkgs, scope);
    let mut pkg_queue = targets
        .into_iter()
        .map(|s| s.as_ref())
        .collect::<VecDeque<&str>>();
    let mut visited = HashSet::new();

    while let Some(pkg) = pkg_queue.pop_front() {
        if !visited.contains(pkg) {
            visited.insert(pkg);
            graph.iter().for_each(|(from, to)| {
                if let Some(from) = from {
                    if *from == pkg {
                        pkg_queue.push_back(to);
                    }
                }
            });
        }
    }
    let targets = targets
        .into_iter()
        .map(|s| s.as_ref())
        .collect::<HashSet<_>>();
    visited
        .into_iter()
        .filter(|pkg| !targets.contains(*pkg))
        .collect()
}

/// for a package graph of a->b->c (where b depends on a), transitive providers of c are a & b and their providers (or what is needed to satisfy c)
pub fn get_transitive_providers<T: AsRef<str>>(targets: &[T], pkgs: &PackageInfos) -> Vec<String> {
    let graph = get_package_graph(pkgs);
    let mut pkg_queue = targets
        .into_iter()
        .map(|s| s.as_ref())
        .collect::<VecDeque<&str>>();
    let mut visited = HashSet::new();

    while let Some(pkg) = pkg_queue.pop_front() {
        if !visited.contains(pkg) {
            visited.insert(pkg);
            graph.iter().for_each(|(from, to)| {
                if let Some(from) = from {
                    if *to == pkg {
                        pkg_queue.push_back(from);
                    }
                }
            });
        }
    }
    let targets = targets
        .into_iter()
        .map(|s| s.as_ref())
        .collect::<HashSet<_>>();
    visited
        .into_iter()
        .filter(|pkg| !targets.contains(*pkg))
        .map(|s| s.to_string())
        .collect()
}

/// Get deps of target package included in giving packages.
pub fn get_internal_deps<'a, 'b>(
    target: &'a PackageInfo,
    packages: &'b PackageInfos,
) -> Vec<&'b str> {
    let deps = target
        .dependencies
        .keys()
        .chain(target.dev_dependencies.keys())
        .collect::<HashSet<_>>();
    packages
        .keys()
        .filter(|pkg| deps.contains(pkg))
        .map(|s| s.as_str())
        .collect()
}
