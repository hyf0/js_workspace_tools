use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use once_cell::sync::Lazy;
use regex::Regex;

/// Searches all package names based on "scoping" (i.e. "scope" in the sense of inclusion)
/// * NOTE: scoping is different than package scopes (@scope/package)
pub fn get_scoped_packages(search: &[String], packages: &[String]) -> Vec<String> {
    // TODO: tests
    let package_names = packages;

    let mut results = HashSet::new();

    // // perform a package-scoped search (e.g. search is @scope/foo*)
    let scoped_search = search
        .iter()
        .filter(|needle| needle.starts_with("@") || needle.starts_with("!@"))
        .map(|s| s.clone())
        .collect::<Vec<_>>();

    multimatch(package_names, &scoped_search)
        .iter()
        .filter(|name| {
            scoped_search
                .iter()
                .any(|search| wax::is_match(search, name.as_str()).unwrap())
        })
        .for_each(|s| {
            results.insert(s.to_string());
        });

    let unscoped_search = search
        .iter()
        .filter(|needle| !needle.starts_with("@") && !needle.starts_with("!@"))
        .collect::<Vec<&String>>();

    if unscoped_search.len() > 0 {
        let bare_pkg_map = generate_bare_package_map(package_names.clone());

        bare_pkg_map
            .keys()
            .filter(|name| {
                unscoped_search
                    .iter()
                    .any(|search| wax::is_match(search, name.as_str()).unwrap())
            })
            .for_each(|s| {
                if let Some(pkgs) = bare_pkg_map.get(s) {
                    pkgs.iter().for_each(|pkg| {
                        results.insert(pkg.to_string());
                    });
                }
            });
    }

    results.into_iter().map(|s| s.to_owned()).collect()
}

/// Regex for matching `@scope/` in `@scope/packgae-name`.
static PACKAGE_SCOPE_PREFIX_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^@[^/]+/").unwrap());

fn generate_bare_package_map(package_names: &[String]) -> HashMap<String, Vec<String>> {
    let mut bare_package_map: HashMap<String, Vec<String>> = HashMap::new();

    // create a map of bare package name -> list of full package names
    // NOTE: do not perform bare_package_map lookup if any of the "scopes" arg starts with "@"
    package_names.into_iter().for_each(|pkg| {
        let bare_pkg_name = PACKAGE_SCOPE_PREFIX_RE.replace_all(&pkg, "").to_string();
        let pkgs = bare_package_map
            .entry(bare_pkg_name)
            .or_insert_with(|| vec![]);
        pkgs.push(pkg.to_string());
    });

    bare_package_map
}

pub(crate) fn multimatch<T: AsRef<Path>, U: AsRef<str>>(paths: &[T], search: &[U]) -> Vec<String> {
    // let paths = paths.into_iter().map(|s| s.as_ref()).collect::<Vec<s>>();
    let mut result = HashSet::new();
    paths.into_iter().for_each(|name| {
        search.into_iter().for_each(|search| {
            let search_str = search.as_ref();
            if search_str.starts_with("!") {
                minimatch(&[name.as_ref()], &&search.as_ref()[1..])
                    .into_iter()
                    .for_each(|s| {
                        result.remove(&s);
                    });
            } else {
                minimatch(&[name], &search).into_iter().for_each(|s| {
                    result.insert(s);
                });
            }
        });
    });

    result.into_iter().collect()
}

fn minimatch<T: AsRef<Path>, U: AsRef<str>>(paths: &[T], search: U) -> Vec<String> {
    // let paths = paths.into_iter().map(|s| s.as_ref()).collect::<Vec<s>>();
    paths
        .iter()
        .filter(|path| wax::is_match(search.as_ref(), path.as_ref()).unwrap())
        .map(|s| s.as_ref().to_string_lossy().to_string())
        .collect()
}