// src/main.rs

use crate::paths::PKGSJSON;
use clap::Parser;

mod sets;
mod args;
mod bootstrap;
mod clean;
mod directions;
mod fetch;
mod flags;
mod macros;
mod misc;
mod package;
mod paths;
mod resolvedeps;
mod tracking;

// TODO: Add color to #[command()]
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Source-based LFS package manager",
    long_about,
    arg_required_else_help = true,
    after_help = "If you have any questions, you can DM me on Discord @toxikuu"
)]
struct Args {
    #[arg(short = 'i', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    install: Option<Vec<String>>,

    #[arg(short = 'n', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    install_no_deps: Option<Vec<String>>,

    #[arg(short = 'r', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    remove: Option<Vec<String>>,

    #[arg(short = 'u', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    update: Option<Vec<String>>,

    #[arg(short = 'd', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    dependencies: Option<Vec<String>>,

    #[arg(short = 'p', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    prune: Option<Vec<String>>,

    // function flags
    #[arg(short = 'l', long, value_name = "PACKAGE", num_args = 0.., value_delimiter = ' ')]
    list: Option<Vec<String>>, // TODO: rewrite this without Option<>

    #[arg(short = 'b', long)]
    bootstrap: bool,

    #[arg(short = 's', long)]
    sync: bool,

    #[arg(short = 'S', long)]
    sync_overwrite: bool,

    // generic flags
    #[arg(short = 'v', long)]
    verbose: bool,

    #[arg(short = 'q', long)]
    quiet: bool,

    #[arg(short = 'D', long)]
    download: bool,

    #[arg(short = 'f', long)]
    force: bool,

    #[arg(short = 'c', long)]
    cache: bool,
}

fn main() {
    let mut args = Args::parse();

    if args.update.is_some() {
        args.force = true;
    }

    flags::set_flags(args.verbose, args.quiet, args.download, args.force);
    vpr!(
        "Flags:\nverbose={}\nquiet={}\ndownload={}\nforce={}",
        args.verbose,
        args.quiet,
        args.download,
        args.force
    );

    bootstrap::tmp();
    let mut pkg_list = tracking::load_package_list(PKGSJSON.as_path()).unwrap_or_else(|_| Vec::new());
    let _ = tracking::append_json(&mut pkg_list); // appends any new metafiles to the json

    if args.bootstrap {
        args::bootstrap();
    }

    if args.cache {
        args::cache();
    }

    if args.sync {
        args::sync();
    }

    if args.sync_overwrite {
        args::sync_overwrite();
    }

    if let Some(pkgs) = args.list {
        args::list(pkgs);
    }

    if let Some(pkgs) = args.remove {
        args::remove(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.prune {
        args::prune(pkgs);
    }

    if let Some(pkgs) = args.install_no_deps {
        args::install_no_deps(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.install {
        args::install(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.update {
        args::update(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.dependencies {
        args::dependencies(pkgs);
    }
}
