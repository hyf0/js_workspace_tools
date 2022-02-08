use std::collections::HashMap;

use crate::{get_internal_deps, structs::package_info::{PackageInfo, PackageInfos}};

mod get_transitive_consumers {
    use std::collections::HashMap;

    use crate::{
        dependencies::{get_transitive_consumers, get_transitive_consumers_with_scope},
        tests::dependencies::stub_package,
    };

    use super::make_pkgs;

    #[test]
    fn can_get_linear_transitive_consumers() {
        //    a -> b -> c
        let all_packages = vec![
            ("a", stub_package("a", &["b"])),
            ("b", stub_package("b", &["c"])),
            ("c", stub_package("c", &[])),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<HashMap<_, _>>();

        let actual = get_transitive_consumers(&["c"], &all_packages);
        assert!(actual.contains(&"a".to_string()));
        assert!(actual.contains(&"b".to_string()));
    }

    #[test]
    fn can_get_linear_transitive_consumers_with_scope() {
        //             demo
        //            /    \
        //        grid      word
        //            \      /  
        //            foo  bar
        //              \  / 
        //              core
        let all_packages = vec![
            ("grid", stub_package("grid", &["foo"])),
            ("word", stub_package("word", &["bar"])),
            ("foo", stub_package("foo", &["core"])),
            ("bar", stub_package("bar", &["core"])),
            ("core", stub_package("core", &[])),
            ("demo", stub_package("demo", &["grid", "word"])),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<HashMap<_, _>>();

        let actual =
            get_transitive_consumers_with_scope(&["core"], &all_packages, &["grid", "word"]);

        assert!(actual.contains(&"foo".to_string()));
        assert!(actual.contains(&"bar".to_string()));
        assert!(actual.contains(&"grid".to_string()));
        assert!(actual.contains(&"word".to_string()));
        assert!(!actual.contains(&"demo".to_string()));
    }

    #[test]
    fn can_get_transitive_consumer_with_deps() {
        /*
        [b, a]
        [d, a]
        [c, b]
        [e, b]
        [f, d]
        [c, g]

        expected: a, b, g (orignates from c)
        */
        let all_packages = make_pkgs(vec![
            ("a", &["b", "d"]),
            ("b", &["c", "e"]),
            ("c", &[]),
            ("d", &["f"]),
            ("e", &[]),
            ("f", &[]),
            ("g", &["c"]),
        ]);

        let actual = get_transitive_consumers(&["c"], &all_packages);

        assert!(actual.contains(&"a".to_string()));
        assert!(actual.contains(&"b".to_string()));
        assert!(actual.contains(&"g".to_string()));

        assert!(!actual.contains(&"d".to_string()));
        assert!(!actual.contains(&"e".to_string()));
        assert!(!actual.contains(&"f".to_string()));
        assert!(!actual.contains(&"c".to_string()));
    }
}

mod get_transitive_providers {
    use crate::dependencies::{get_transitive_consumers_with_scope, get_transitive_providers};

    use super::*;
    #[test]
    fn can_get_linear_transitive_providers() {
        let all_packages = make_pkgs(vec![("a", &["b"]), ("b", &["c"]), ("c", &[])]);

        let actual = get_transitive_providers(&["a"], &all_packages);

        assert!(actual.contains(&"b".to_string()));
        assert!(actual.contains(&"c".to_string()));
    }

    #[test]
    fn can_get_transitive_providers_with_deps() {
        /*
          [b, a]
          [c, b]
          [e, c]
          [f, c]
          [f, e]
          [g, f]

          expected: e, f, g
        */
        let all_packages = make_pkgs(vec![
            ("a", &["b"]),
            ("b", &["c"]),
            ("c", &["e", "f"]),
            ("d", &[]),
            ("e", &["f"]),
            ("f", &["g"]),
            ("g", &[]),
        ]);

        let actual = get_transitive_providers(&["c"], &all_packages);

        assert!(actual.contains(&"e".to_string()));
        assert!(actual.contains(&"f".to_string()));
        assert!(actual.contains(&"g".to_string()));

        assert!(!actual.contains(&"a".to_string()));
        assert!(!actual.contains(&"b".to_string()));
        assert!(!actual.contains(&"d".to_string()));
        assert!(!actual.contains(&"c".to_string()));
    }

    #[test]
    fn can_get_transitive_consumers_with_deps_and_scope() {
        /*
          [b, a]
          [c, b]
          [e, c]
          [f, c]
          [f, e]
          [g, f]

          expected: e, f, g
        */

        let all_packages = make_pkgs(vec![
            ("a", &["b", "h"]),
            ("b", &["c"]),
            ("c", &["e", "f"]),
            ("d", &[]),
            ("e", &["f"]),
            ("f", &["g"]),
            ("g", &[]),
            ("h", &["i"]),
            ("i", &["f"]),
        ]);

        let actual = get_transitive_consumers_with_scope(&["f"], &all_packages, &["b"]);

        assert!(actual.contains(&"e".to_string()));
        assert!(actual.contains(&"c".to_string()));
        assert!(actual.contains(&"b".to_string()));

        assert!(!actual.contains(&"h".to_string()));
    }
}

#[test]
fn test_get_internal_deps() {
  let info = stub_package("a", &["b", "c"]);
  let packages = make_pkgs(vec![("a", &["b", "c"]), ("b", &["c"]), ("c", &[]), ("d", &[])]);
  let deps = get_internal_deps(&info, &packages);
  assert!(!deps.contains(&"a"));
  assert!(deps.contains(&"b"));
  assert!(deps.contains(&"c"));
  assert!(!deps.contains(&"d"));
}

fn stub_package(name: &str, deps: &[&str]) -> PackageInfo {
    PackageInfo {
        name: name.to_string(),
        __filename: format!("packages/{}", name),
        version: "1.0".to_string(),
        dependencies: deps
            .into_iter()
            .fold(Default::default(), |mut dep_map, dep| {
                dep_map.insert(dep.to_string(), "*".to_string());
                dep_map
            }),
        ..Default::default()
    }
}

fn make_pkgs(pkgs: Vec<(&str, &[&str])>) -> PackageInfos {
    pkgs.into_iter()
        .map(|p| stub_package(p.0, p.1))
        .map(|value| (value.name.clone(), value))
        .collect::<HashMap<_, _>>()
}
