use clap::Parser;
use git2::Repository;
use std::path::Path;
mod repo;
mod tree;
use repo::{explore_path, print_long, print_long_tree, print_summary, RepositoryStatus};

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

    /// Display summary format
    #[arg(short, long)]
    summary: bool,

    /// Display clean repositories
    #[arg(short, long)]
    clean: bool,

    /// Display in tree form
    #[arg(short, long)]
    tree: bool,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path).canonicalize().unwrap();

    let mut repositories: Vec<Repository> = vec![];
    explore_path(&path, &mut repositories);

    println!("{}", path.to_str().unwrap());

    let repository_statuses: Vec<RepositoryStatus> = repositories
        .iter()
        .map(|repo| RepositoryStatus::new(repo, &path))
        .collect();

    if !args.summary {
        if args.tree {
            print_long_tree(repository_statuses, args.url, args.clean);
        } else {
            print_long(repository_statuses, args.url, args.clean);
        }
    } else {
        print_summary(repository_statuses);
    }
}
