extern crate bindgen;
extern crate git2;
extern crate regex;

use git2::Repository;
use regex::Regex;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const LIBINJECTION_URL: &'static str = "https://github.com/client9/libinjection";
const BUILD_DIR_NAME: &'static str = "libinjection";

fn clone_libinjection(build_dir: &Path, version: &str) -> Option<()> {
    let repo = Repository::clone(LIBINJECTION_URL, build_dir).ok()?;
    let rev = repo.revparse_single(version).ok()?;
    repo.set_head_detached(rev.id()).ok()
}

fn run_make(rule: &str, cwd: &Path) -> Option<bool> {
    let output = Command::new("make")
        .arg(rule)
        .env("OUT_DIR", env::var("OUT_DIR").unwrap())
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
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut build_parent_dir = out_path.join(BUILD_DIR_NAME);

    if clone_libinjection(build_parent_dir.as_path(), "v3.10.0").is_none() {
        panic!("unable to clone libinjection");
    }

    if fix_python_version().is_none() {
        panic!("unable to fix python version");
    }

    build_parent_dir.push("src");
    if let Some(success) = run_make("all", build_parent_dir.as_path()) {
        if !success {
            panic!("unable to make libinjection");
        }
    } else {
        panic!("unable to make libinjection");
    }

    println!("cargo:rustc-link-lib=static=injection");
    println!("cargo:rustc-link-search={}", build_parent_dir.display());

    let h_path = build_parent_dir.join("libinjection.h");
    let bindings = bindgen::Builder::default()
        .header(h_path.to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings");
}
