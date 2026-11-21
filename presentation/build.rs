use std::fs;
use std::hash::{DefaultHasher, Hasher};
use std::path::{Path, PathBuf};

const ASSET_SOURCES: [&str; 5] = [
    "css",
    "js",
    "templates",
    "../package-lock.json",
    "../vite.config.mjs",
];

fn main() {
    let mut files = Vec::new();

    for source in ASSET_SOURCES {
        println!("cargo::rerun-if-changed={source}");
        collect_files(Path::new(source), &mut files);
    }

    files.sort();

    let mut hasher = DefaultHasher::new();

    for file in files {
        let contents = fs::read(&file)
            .unwrap_or_else(|error| panic!("cannot read [{}]: {error}", file.display()));
        hasher.write(file.to_string_lossy().as_bytes());
        hasher.write(&contents);
    }

    println!("cargo::rustc-env=ASSET_VERSION={:016x}", hasher.finish());
}

fn collect_files(path: &Path, files: &mut Vec<PathBuf>) {
    if path.is_file() {
        files.push(path.to_path_buf());
        return;
    }

    let entries = fs::read_dir(path)
        .unwrap_or_else(|error| panic!("cannot read [{}]: {error}", path.display()));

    for entry in entries {
        let entry = entry.unwrap_or_else(|error| {
            panic!("cannot read an entry of [{}]: {error}", path.display())
        });
        collect_files(&entry.path(), files);
    }
}
