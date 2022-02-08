use crate::get_scoped_packages;

#[test]
fn can_match_scopes_for_full_matches_for_an_array() {
    let results = get_scoped_packages(
        &["foo", "bar"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        &["foo", "bar", "baz"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );
    assert!(results.contains(&"foo".to_owned()));
    assert!(results.contains(&"bar".to_owned()));
    assert!(!results.contains(&"baz".to_owned()));
}

#[test]
fn can_match_with_wildcards() {
    let results = get_scoped_packages(
        &["foo*"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        &["foo1", "foo2", "baz"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );
    assert!(results.contains(&"foo1".to_owned()));
    assert!(results.contains(&"foo2".to_owned()));
    assert!(!results.contains(&"baz".to_owned()));
}

#[test]
fn can_match_with_npm_package_scopes() {
    let results = get_scoped_packages(
        &["foo*"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        &["@yay/foo1", "@yay1/foo2", "foo", "baz"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );
    assert!(results.contains(&"@yay/foo1".to_owned()));
    assert!(results.contains(&"@yay1/foo2".to_owned()));
    assert!(results.contains(&"foo".to_owned()));
    assert!(!results.contains(&"baz".to_owned()));
}

#[test]
fn uses_the_correct_package_scope_when_the_search_pattern_starts_a_character() {
    let results = get_scoped_packages(
        &["@yay/foo*"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        &["@yay/foo1", "@yay1/foo2", "foo", "baz"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );
    assert!(results.contains(&"@yay/foo1".to_owned()));
    assert!(!results.contains(&"@yay1/foo2".to_owned()));
    assert!(!results.contains(&"foo".to_owned()));
    assert!(!results.contains(&"baz".to_owned()));
}

#[test]
fn can_deal_with_brace_expansion_with_scopes() {
    let results = get_scoped_packages(
        &["@yay/foo{1,2}"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        &["@yay/foo1", "@yay/foo2", "@yay/foo3", "foo", "baz"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );
    assert!(results.contains(&"@yay/foo1".to_owned()));
    assert!(results.contains(&"@yay/foo2".to_owned()));
    assert!(!results.contains(&"@yay/foo3".to_owned()));
    assert!(!results.contains(&"foo".to_owned()));
    assert!(!results.contains(&"baz".to_owned()));
}

#[test]
fn can_deal_with_negated_search() {  
    let results = get_scoped_packages(
        &["@yay/foo*", "!@yay/foo3"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        &["@yay/foo1", "@yay/foo2", "@yay/foo3", "foo", "baz"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );
    assert!(results.contains(&"@yay/foo1".to_owned()));
    assert!(results.contains(&"@yay/foo2".to_owned()));
    assert!(!results.contains(&"@yay/foo3".to_owned()));
    assert!(!results.contains(&"foo".to_owned()));
    assert!(!results.contains(&"baz".to_owned()));
}
