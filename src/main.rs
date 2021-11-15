use anyhow::Result;
use log::*;
use simplelog::*;
// use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

mod settings;
use crate::settings::Settings;
mod utils;
use crate::utils::tilde_path;
mod source_linker;
use crate::source_linker::SourceLinker;

#[derive(StructOpt)]
/// Sprinkle!
///
/// Sprinkle your dotfiles all around
#[structopt(name = "sprinkle", about = "Dotfile manager", author)]
struct Cli {
    /// Output file
    #[structopt(
        short = "h",
        long = "home-dir-override",
        help = "Override $HOME",
        default_value = "~",
        parse(from_os_str)
    )]
    home_dir_override: PathBuf,
}

fn main() -> Result<()> {
    let settings = Settings::new()?;

    let term_filter = match settings.debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Warn,
    };
    CombinedLogger::init(vec![
        #[cfg(feature = "termcolor")]
        TermLogger::new(term_filter, Config::default(), TerminalMode::Mixed).unwrap(),
        #[cfg(not(feature = "termcolor"))]
        SimpleLogger::new(term_filter, Config::default()),
    ])?;

    let args = Cli::from_args();
    let home_dir = tilde_path(&args.home_dir_override);
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
        );
        debug!("{:?}", sl);
        sl.link()?;
    }

    Ok(())
}
