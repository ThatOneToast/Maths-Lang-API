use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use git2::Repository;

pub struct Maths {
    pub executable: String,
}

impl Maths {
    pub fn new(override_executable: Option<String>) -> Self {
        if let Some(executable) = override_executable {
            Self { executable }
        } else {
            let executable = build_return_executable();
            Self { executable }
        }
    }

    pub fn run(&self, input: &str) {
        let mut command = Command::new(&self.executable);
        command.arg("interp");

        let mut child = command
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(input.as_bytes())
                .expect("failed to write to stdin");
        }

        let output = child.wait_with_output().expect("failed to wait on child");
        println!("Output: {:?}", String::from_utf8(output.stdout).unwrap());
    }
}

/// Cleans repos, clones it, and returns the executable
pub fn build_return_executable() -> String {
    clean_repos();
    clone_repo("https://github.com/ThatOneToast/Maths-Lang.git");
    build_repo();
    let repo = Path::new("./ml-api").join("Maths-Lang");
    let debug = repo.join("target/debug/");
    let maths = debug.join("Maths");

    let maths_str = maths.to_str().unwrap();

    maths_str.to_string()
}

pub fn clone_repo(url: &str) {
    println!("Cloning repo: {}", url);
    let git_name = url.split("/").collect::<Vec<&str>>().pop().unwrap();
    let repo_name = git_name.replace(".git", "");
    match Repository::clone(url, format!("./ml-api/{}", repo_name)) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

pub fn clean_repos() {
    // if the ml-api folder exists, delete it
    if Path::new("./ml-api").exists() {
        std::fs::remove_dir_all("./ml-api").unwrap();
    }
}

pub fn build_repo() {
    let repo = Path::new("./ml-api").join("Maths-Lang");
    
    let mut build_command = Command::new("cargo")
        .arg("build")
        .current_dir(repo)
        .spawn()
        .expect("failed to execute process");

    build_command.wait().expect("failed to wait on child");
}
