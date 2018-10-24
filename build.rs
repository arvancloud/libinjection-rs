extern crate git2;
extern crate regex;

use git2::Repository;
use regex::Regex;
use std::env;
use std::path::Path;
use std::process::Command;

const LIBINJECTION_URL: &'static str = "https://github.com/client9/libinjection";
const BUILD_DIR_NAME: &'static str = "libinjection";

fn clone_libinjection(build_dir: &Path, version: &str) -> Option<()> {
    let repo = if build_dir.exists() {
        Repository::open(build_dir).ok()?
    } else {
        Repository::clone(LIBINJECTION_URL, build_dir).ok()?
    };
    let rev = repo.revparse_single(version).ok()?;
    repo.set_head_detached(rev.id()).ok()
}

fn run_make(rule: &str, cwd: &Path) -> Option<bool> {
    let output = Command::new("make")
        .arg(rule)
        .current_dir(cwd)
        .output()
        .ok()?;
    Some(output.status.success())
}

fn fix_python_version() -> Option<()> {
    let output = Command::new("python").arg("-V").output().ok()?;
    let python_version = String::from_utf8_lossy(&output.stdout).to_string();
    if !Regex::new("Python 2.*")
        .ok()?
        .is_match(python_version.as_str())
    {
        let cwd = env::current_dir().ok()?;
        if let Some(success) = run_make("fix-python", cwd.as_path()) {
            if !success {
                return None;
            }
        } else {
            return None;
        }
    }
    Some(())
}

fn main() {
    let mut build_parent_dir = env::current_dir().unwrap();
    build_parent_dir.push(BUILD_DIR_NAME);
    let build_dir = build_parent_dir.as_path();

    if clone_libinjection(build_dir, "v3.10.0").is_none() {
        panic!("unable to clone libinjection");
    }

    if fix_python_version().is_none() {
        panic!("unable to fix python version");
    }
}
