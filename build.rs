extern crate git2;

use git2::Repository;
use std::env;
use std::path::Path;

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

fn main() {
    let mut build_parent_dir = env::current_dir().unwrap();
    build_parent_dir.push(BUILD_DIR_NAME);
    let build_dir = build_parent_dir.as_path();

    if clone_libinjection(build_dir, "v3.10.0").is_none() {
        panic!("unable to clone libinjection");
    }
    // TODO
}
