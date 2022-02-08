use std::{
    ffi::OsStr,
    process::{self, Command},
};

#[derive(Debug)]
pub struct ProcessOutput {
    pub stderr: String,
    pub stdout: String,
    pub success: bool,
}

impl From<process::Output> for ProcessOutput {
    fn from(output: process::Output) -> Self {
        Self {
            stderr: String::from_utf8(output.stderr)
                .unwrap()
                .trim_end()
                .to_string(),
            stdout: String::from_utf8(output.stdout)
                .unwrap()
                .trim_end()
                .to_string(),
            success: output.status.success(),
        }
    }
}

pub fn git_with<T: AsRef<OsStr>>(args: &[T], cwd: &str) -> ProcessOutput {
    let results: ProcessOutput = Command::new("git")
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("failed to execute process")
        .into();
    results
}

pub fn get_default_branch(cwd: &str) -> String {
    let result = git_with(&["config", "init.defaultBranch"], cwd);
    if result.success {
        result.stdout.trim().to_string()
    } else {
        "master".to_string()
    }
}
