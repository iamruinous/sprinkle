// SPDX-FileCopyrightText: © 2022 Jade Meskill
//
// SPDX-License-Identifier: MIT

use anyhow::Result;
use clap::Parser;
use log::*;
use simplelog::*;
use std::path::PathBuf;

mod settings;
use crate::settings::Settings;
mod utils;
use crate::utils::tilde_path;
mod source_linker;
use crate::source_linker::SourceLinker;

#[derive(Parser, Debug)]
/// Sprinkle!
///
/// Sprinkle your dotfiles all around
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Output file
    #[clap(
        short = 'o',
        long = "home-dir-override",
        help = "Override $HOME",
        default_value = "~",
        env = "SPRINKLE_HOME_DIR_OVERRIDE",
        parse(from_os_str)
    )]
    home_dir_override: PathBuf,

    /// Overwrite symlinks
    #[clap(short, long, env = "SPRINKLE_FORCE_OVERWRITE")]
    force_overwrite: bool,
}

fn main() -> Result<()> {
    let settings = Settings::new()?;

    let term_filter = match settings.debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Warn,
    };
    CombinedLogger::init(vec![
        #[cfg(feature = "termcolor")]
        TermLogger::new(
            term_filter,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        #[cfg(not(feature = "termcolor"))]
        SimpleLogger::new(term_filter, Config::default()),
    ])?;

    let args = Cli::parse();
    let home_dir = tilde_path(&args.home_dir_override);
    let force_overwrite = args.force_overwrite;
    let sources = settings.sources;
    let excludes = settings.excludes;
    for (name, source) in sources.iter() {
        let sl = SourceLinker::new(
            &home_dir,
            name,
            source.enabled,
            &source.path,
            &source.excludes,
            &excludes,
            force_overwrite,
        );
        debug!("{:?}", sl);
        sl.link()?;
    }

    Ok(())
}
