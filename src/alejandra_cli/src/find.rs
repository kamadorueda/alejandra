use std::collections::HashSet;
use std::process::exit;

pub(crate) fn nix_files(
    include: &[&str],
    exclude: &[String],
    respect_gitignore: bool,
) -> Vec<String> {
    let include: HashSet<_> = include
        .iter()
        .flat_map(|s| nix_files_in_path(s, respect_gitignore))
        .collect();
    let exclude: HashSet<_> =
        exclude.iter().flat_map(|s| nix_files_in_path(s, false)).collect();

    let mut paths: Vec<_> = include
        .difference(&exclude)
        .cloned()
        .map(|path| (len_of(&path), path))
        .collect();

    // Sort by file size descending: larger files first.
    // This is a work-stealing heuristic for the thread pool — scheduling large
    // files first ensures the pool stays busy while smaller files finish,
    // improving overall throughput.
    paths.sort_unstable_by(|(len_a, _), (len_b, _)| len_b.cmp(len_a));

    paths.into_iter().map(|(_, path)| path).collect()
}

/// The `.nix` files in `path` (a file or a directory).
///
/// With `respect_gitignore`, files and directories that `.gitignore` files
/// (also those of parent directories, `.git/info/exclude` and the global
/// git ignore file) ignore are skipped. A path given directly is never
/// skipped. Hidden files are not skipped, and symbolic links are not
/// followed, as without it.
fn nix_files_in_path(path: &str, respect_gitignore: bool) -> HashSet<String> {
    ignore::WalkBuilder::new(path)
        .standard_filters(false)
        .git_ignore(respect_gitignore)
        .git_global(respect_gitignore)
        .git_exclude(respect_gitignore)
        .parents(respect_gitignore)
        // Also outside of a git repository, like in a release tarball
        .require_git(false)
        .build()
        .filter_map(|entry| entry.ok())
        .filter(is_nix_file)
        .map(to_full_path)
        .collect()
}

fn is_nix_file(entry: &ignore::DirEntry) -> bool {
    entry.file_type().is_some_and(|file_type| file_type.is_file())
        && entry.file_name().to_str().unwrap().ends_with(".nix")
}

fn to_full_path(entry: ignore::DirEntry) -> String {
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
