use std::collections::HashSet;
use std::process::exit;

pub(crate) fn nix_files(include: &[&str], exclude: &[String]) -> Vec<String> {
    let include: HashSet<_> =
        include.iter().flat_map(|s| nix_files_in_path(s)).collect();
    let exclude: HashSet<_> =
        exclude.iter().flat_map(|s| nix_files_in_path(s)).collect();

    let mut paths: Vec<_> = include
        .difference(&exclude)
        .cloned()
        .map(|path| (len_of(&path), path))
        .collect();

    paths.sort_unstable_by(|(len_a, _), (len_b, _)| len_b.cmp(len_a));

    paths.into_iter().map(|(_, path)| path).collect()
}

fn nix_files_in_path(path: &str) -> HashSet<String> {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_entry(is_nix_file_or_dir)
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry),
            Err(_) => None,
        })
        .filter(is_nix_file)
        .map(to_full_path)
        .collect()
}

fn is_nix_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
        && entry.file_name().to_str().unwrap().ends_with(".nix")
}

fn is_nix_file_or_dir(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_dir() || is_nix_file(entry)
}

fn to_full_path(entry: walkdir::DirEntry) -> String {
    entry.path().to_str().unwrap().to_string()
}

fn len_of(path: &str) -> u64 {
    match std::fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(err) => {
            eprintln!("Could not get the size of file at: {path}");
            eprintln!("Got error: {err}");
            exit(1);
        }
    }
}
