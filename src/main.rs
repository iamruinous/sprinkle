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

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
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
        // WriteLogger::new(
        //     LevelFilter::Info,
        //     Config::default(),
        //     File::create("my_rust_binary.log").unwrap(),
        // ),
    ])?;

    let args = Cli::from_args();
    let home_dir = tilde_path(&args.home_dir_override);
    let sources = settings.sources;
    let excludes = settings.excludes;
    for (name, source) in sources.iter() {
        let sl = SourceLinker::new(
            home_dir.clone(),
            name.clone(),
            source.enabled,
            source.path.clone(),
            source.excludes.clone(),
            excludes.clone(),
        );
        debug!("{:?}", sl);
        sl.link()?;
    }

    Ok(())
}
