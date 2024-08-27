use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use git2::Repository;

pub struct Maths {
    pub executable: String,
    result: Option<f64>
}

impl Maths {
    /// Creqates a new Maths object that can be used to run maths files.
    /// If `override_executable` is provided, it will use that executable.
    /// Otherwise, it will clone and build the repo and use that executable.
    pub fn new(override_executable: Option<String>) -> Self {
        if let Some(executable) = override_executable {
            Self { executable, result: None }
        } else {
            let executable = build_return_executable();
            Self { executable, result: None }
        }
    }

    /// Takes in a maths file string inpput, and runs it.
    /// Get the final result with `get_result()`
    pub fn run(&mut self, input: &str) {
        let mut command = Command::new(&self.executable);
        command.arg("interp");
    
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
    
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input.as_bytes())
                .expect("failed to write to stdin");
            stdin.flush().expect("failed to flush stdin");
        }
    
        let output = child
            .wait_with_output()
            .expect("failed to wait on child process");
    
        let stdout = String::from_utf8_lossy(&output.stdout);
    
        if let Some(result_line) = stdout.lines().find(|line| line.starts_with("result=")) {
            let final_result = result_line.replace("result=", "");
            let final_num = final_result.parse::<f64>().expect("failed to parse result");
            self.result = Some(final_num);
        } else {
            println!("No result captured");
        }
    }
    
    /// Returns the final result of the maths file
    pub fn get_result(&self) -> Option<f64> {
        self.result
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
