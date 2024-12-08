// args.rs
//
// defines args

use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Source-based LFS package manager",
    long_about,
    arg_required_else_help = true,
    after_help = "If you have any questions, you can DM me on Discord @toxikuu"
)]
pub struct Args {
    // Core flags (independent of package arguments)
    #[arg(short = 'i', long, action = ArgAction::SetTrue)]
    pub install: bool,

    #[arg(short = 'I', long, action = ArgAction::SetTrue)]
    pub install_with_dependencies: bool,

    #[arg(short = 'r', long, action = ArgAction::SetTrue)]
    pub remove: bool,

    #[arg(short = 'R', long, action = ArgAction::SetTrue)]
    pub remove_with_dependencies: bool,

    #[arg(short = 'u', long, action = ArgAction::SetTrue)]
    pub update: bool,

    #[arg(short = 'U', long, action = ArgAction::SetTrue)]
    pub update_with_dependencies: bool,

    #[arg(short = 'd', long, action = ArgAction::SetTrue)]
    pub dependencies: bool,

    #[arg(short = 'D', long, action = ArgAction::SetTrue)]
    pub dependants: bool,

    #[arg(short = 'p', long, action = ArgAction::SetTrue)]
    pub prune: bool,

    #[arg(short = 'g', long, action = ArgAction::SetTrue)]
    pub get: bool,

    #[arg(short = 's', long, action = ArgAction::SetTrue)]
    pub search: bool,

    // #[arg(short = 'G', long, action = ArgAction::SetTrue)]
    // pub force_get_files: bool,

    #[arg(short = 'l', long, action = ArgAction::SetTrue)]
    pub list: bool,

    #[arg(short = 'L', long, action = ArgAction::SetTrue)]
    pub list_outdated: bool,

    #[arg(short = 'n', long, action = ArgAction::SetTrue)]
    pub news: bool,

    // Function flags
    #[arg(short = 'c', long, action = ArgAction::SetTrue)]
    pub cache: bool,

    #[arg(short = 'k', long, action = ArgAction::SetTrue)]
    pub check_upstream: bool,

    #[arg(long, action = ArgAction::SetTrue)]
    pub validate_links: bool,

    #[arg(short = 'S', long, action = ArgAction::SetTrue)]
    pub sync: bool,

    // Generic flags
    #[arg(short = 'v', long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    #[arg(short = 'q', long, action = ArgAction::SetTrue)]
    pub quiet: bool,

    #[arg(short = 'f', long, action = ArgAction::SetTrue)]
    pub force: bool,

    // Positional arguments for packages
    #[arg(value_name = "PACKAGE", num_args = 0.., value_delimiter = ' ')]
    pub packages: Vec<String>,
}

pub fn init_args() -> Args {
    Args::parse()
}
