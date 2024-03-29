// SPDX-FileCopyrightText: © 2022 Jade Meskill
//
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::path::PathBuf;

use sprinkle::source_linker::SourceLinker;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = PathBuf::new();
    let name = "test";
    let enabled = true;
    let path = PathBuf::new();
    let excludes: HashSet<PathBuf> = HashSet::new();
    let global_excludes: HashSet<PathBuf> = HashSet::new();
    let sl = SourceLinker::new(
        &home_dir,
        &name,
        enabled,
        &path,
        &excludes,
        &global_excludes,
        false,
    );
    let _ = sl.link();

    Ok(())
}
