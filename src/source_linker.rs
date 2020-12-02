use anyhow::Result;
use log::*;
use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::utils::{is_not_excluded, is_not_hidden, tilde_path};

#[derive(Debug)]
pub struct SourceLinker {
    home_dir: PathBuf,
    name: String,
    enabled: bool,
    excludes: HashSet<PathBuf>,
    global_excludes: HashSet<PathBuf>,
    path: PathBuf,
}

impl SourceLinker {
    pub fn new(
        home_dir: PathBuf,
        name: String,
        enabled: bool,
        path: PathBuf,
        excludes: HashSet<PathBuf>,
        global_excludes: HashSet<PathBuf>,
    ) -> SourceLinker {
        SourceLinker {
            home_dir,
            name,
            enabled,
            path,
            excludes,
            global_excludes,
        }
    }

    fn link_file(&self, entry: &DirEntry, dest_path: &Path) -> Result<()> {
        if dest_path.exists() {
            info!(
                "symlink exists, skipping {}, {}",
                entry.path().display(),
                dest_path.display()
            );
            return Ok(());
        }
        info!(
            "Symlink {} => {}",
            entry.path().display(),
            dest_path.display()
        );
        let parent_path = dest_path.parent().unwrap();
        if !parent_path.exists() {
            info!("creating directory {}", parent_path.display());
            fs::create_dir_all(parent_path)?;
        }
        symlink(entry.path(), dest_path)?;
        return Ok(());
    }

    // fn link_dir(&self, entry: &DirEntry, dest_path: &Path) -> Result<()> {
    //     // If directory is empty, skip
    //     if fs::read_dir(entry.path())?.take(1).count() > 0 {
    //         info!("directory is empty, skipping {}", dest_path.display());
    //         return Ok(());
    //     }
    //     // If directory already exists, skip
    //     if dest_path.exists() {
    //         info!("directory exists, skipping {}", dest_path.display());
    //         return Ok(());
    //     }
    //     info!(
    //         "mkdir -p {}, {}",
    //         entry.path().display(),
    //         dest_path.display()
    //     );
    //     fs::create_dir_all(dest_path)?;
    //     return Ok(());
    // }

    pub fn link(&self) -> Result<()> {
        if !self.enabled {
            info!("Skipping {}", self.name);
            return Ok(());
        }
        info!("Working on {}", self.name);
        let source_path = tilde_path(&self.path);
        let walker = WalkDir::new(&source_path);
        for entry in walker
            .into_iter()
            .filter_entry(|e| {
                is_not_hidden(e)
                    && is_not_excluded(e, &source_path, &self.global_excludes)
                    && is_not_excluded(e, &source_path, &self.excludes)
            })
            .filter_map(|e| e.ok())
        {
            if entry.depth() == 0 {
                continue;
            }
            let entry_frag = entry.path().strip_prefix(&source_path)?;
            let dest_file = format!("{}/.{}", self.home_dir.display(), entry_frag.display());
            let dest_path = Path::new(&dest_file);
            if entry.file_type().is_file() {
                self.link_file(&entry, dest_path)?;
            }
            // if entry.file_type().is_dir() {
            //     self.link_dir(&entry, dest_path)?;
            // }
        }
        return Ok(());
    }
}
