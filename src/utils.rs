use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::DirEntry;
pub fn tilde_path(path: &Path) -> PathBuf {
    let st = shellexpand::tilde(path.to_str().unwrap()).to_string();
    return Path::new(&st).to_path_buf();
}

pub fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with('.'))
        .unwrap_or(false)
}

pub fn is_not_excluded(entry: &DirEntry, src_base: &Path, excludes: &HashSet<PathBuf>) -> bool {
    let src = entry.path();
    let src_rel = src.strip_prefix(src_base).unwrap();
    !excludes.contains(src_rel)
}
