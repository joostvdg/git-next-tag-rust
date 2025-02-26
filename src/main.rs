use std::io::Write;
use clap::Parser;
use clap_verbosity_flag::{Verbosity};
use log::{info};
use env_logger::{Builder, Target};

#[macro_use]
extern crate log;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug,Parser)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,

    /// The base of tag version to amend (e.g., 1.10, to find the next patch 1.10.X)
    #[arg(required = true, short = 'b', long = "baseTag")]
    base_tag: String,

    /// The path where to execute the git tag command
    #[arg(required = true, short = 'p', long = "path", default_value = ".")]
    path: std::path::PathBuf,

    /// The path (file) where to output result to
    #[arg(short = 'o', long = "outputPath")]
    output_path: Option<std::path::PathBuf>,

    /// If the Tag has a suffix (e.g., -rc) and you want to find x.y.z-rc-Z with the latest Z for the latest z
    #[arg(short = 's', long = "suffix")]
    tag_suffix: Option<String>,

    /// If the release is a pre-release, and requires a suffix beyond the Major.Minor.Patch
    #[arg(short = 'r', long = "preRelease")]
    pre_release: bool,

    /// If the suffix for a pre-release should be -<commitShaShort>, for example "0.30.0-527e2ab"
    #[arg(short = 'c', long = "commit")]
    commit: bool,
}

fn main()  {
    let args = Cli::parse();

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout)
        .filter_level(args.verbose.log_level_filter())
        .init();
    debug!("Starting up");
    debug!("{:?}", args);

    let path = args.path.to_str().unwrap();
    let suffix = args.tag_suffix.unwrap_or("".to_string());

    // Determine the version type
    let version_type = if args.pre_release {
        if args.commit {
            git_next_tag::VersionType::PreReleaseCommit
        } else {
            git_next_tag::VersionType::PreRelease
        }
    } else {
        git_next_tag::VersionType::Stable
    };

    // Construct the next tag request
    let next_tag_request = git_next_tag::NextTagRequest {
        base_tag: args.base_tag,
        path: path.to_string(),
        suffix: Some(suffix.to_string()),
        version_type,
    };

    let next_tag = git_next_tag::determine_nex_tag(next_tag_request).unwrap();
    info!("Next tag: {}", next_tag);

    if let Some(output_path) = args.output_path {
        let mut file = std::fs::File::create(output_path).unwrap();
        file.write_all(next_tag.as_bytes()).unwrap();
    }

}
