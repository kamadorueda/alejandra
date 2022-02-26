pub(crate) fn nix_files(include: Vec<&str>, exclude: Vec<&str>) -> Vec<String> {
    let include: std::collections::HashSet<String> =
        include.iter().flat_map(nix_files_in_path).collect();
    let exclude: std::collections::HashSet<String> =
        exclude.iter().flat_map(nix_files_in_path).collect();

    let mut paths: Vec<String> =
        include.difference(&exclude).cloned().collect();

    paths.sort();
    paths
}

fn nix_files_in_path(path: &&str) -> std::collections::HashSet<String> {
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
