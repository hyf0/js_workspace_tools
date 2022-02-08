mod pnpm {
    use std::collections::HashMap;

    use crate::get_pnpm_wrokspaces;

    #[test]
    fn gets_the_name_and_path_of_the_workspaces() {
        env_logger::init();
        let package_root = path::join!(&path::resolve!(), "fixtures/monorepo-pnpm");
        let workspaces_package_info = get_pnpm_wrokspaces(&package_root)
            .into_iter()
            .map(|s| (s.name.clone(), s))
            .collect::<HashMap<_, _>>();
        let package_a_path = path::join!(&package_root, "packages", "package-a");
        let package_b_path = path::join!(&package_root, "packages", "package-b");
        let workspace_a = workspaces_package_info.get("package-a").unwrap();
        let workspace_b = workspaces_package_info.get("package-b").unwrap();
        assert_eq!(workspace_a.path, package_a_path);
        assert_eq!(workspace_a.name, "package-a");
        assert_eq!(workspace_b.path, package_b_path);
        assert_eq!(workspace_b.name, "package-b");
    }
}
