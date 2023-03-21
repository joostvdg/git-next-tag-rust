use clap::Parser;
use clap_verbosity_flag::{Verbosity};
use anyhow::{Context, Result};
use log::{info};
use env_logger::{Builder, Target};

#[macro_use]
extern crate log;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug,Parser)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,

    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main()  -> Result<()> {
    let args = Cli::parse();

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout)
        .filter_level(args.verbose.log_level_filter())
        .init();
    info!("Starting up");
    debug!("{:?}", args);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    debug!("Reading file {}", &args.path.display());
    git_next_tag::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}



