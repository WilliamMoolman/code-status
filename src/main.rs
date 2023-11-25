use clap::Parser;
use git2::Repository;
use std::path::Path;
mod repo;
use repo::{explore_path, print_long, print_summary, RepositoryStatus};

/// A CLI tool to get the status of many git repositories
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The root level folder to begin searching
    #[arg(default_value_t = String::from("."))]
    path: String,

    /// Display origin url
    #[arg(short, long)]
    url: bool,

    /// Display long format
    #[arg(short, long)]
    long: bool,

    /// Display clean repositories
    #[arg(short, long)]
    clean: bool,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);

    let mut repositories: Vec<Repository> = vec![];
    explore_path(path, &mut repositories);

    let repository_statuses: Vec<RepositoryStatus> =
        repositories.iter().map(RepositoryStatus::new).collect();

    if args.long {
        print_long(repository_statuses, args.url, args.clean);
    } else {
        print_summary(repository_statuses);
    }
}
