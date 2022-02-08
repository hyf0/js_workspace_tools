mod get_default_branch {
    use crate::git::{get_default_branch, git_with};

    #[ignore = "FIXME:"]
    #[test]
    fn is_main_in_the_default_test_repo() {
        let cwd = path::join!(&path::resolve!(), "fixtures/basic");
        let branch = get_default_branch(&cwd);
        assert_eq!(branch, "main")
    }

    #[test]
    fn is_my_main_when_default_branch_is_different() {
        // WARN: this test has side-effect.
        let cwd = path::join!(&path::resolve!(), "fixtures/basic");
        git_with(&["config", "init.defaultBranch", "main"], &cwd);
        let branch = get_default_branch(&cwd);
        assert_eq!(branch, "main")
    }
}
