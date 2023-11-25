use git2::{Repository, Statuses};
use std::path::Path;

pub struct RepositoryStatus {
    name: String,
    status_flags: String,
    origin_url: String,
}

impl RepositoryStatus {
    pub fn new(repo: &Repository) -> RepositoryStatus {
        let statuses = repo.statuses(None).unwrap();
        let name = repo
            .path()
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        let origin_url = match repo.find_remote("origin") {
            Ok(ref remote) => remote
                .url()
                .expect("URL for origin could not be found")
                .to_owned(),
            Err(_) => String::new(),
        };
        let status_flags = statuses_to_string(statuses);

        RepositoryStatus {
            name,
            status_flags,
            origin_url,
        }
    }
}

struct StatusFlags {
    new: bool,
    modified: bool,
    deleted: bool,
    renamed: bool,
    push: bool,
    pull: bool,
}

impl StatusFlags {
    fn new() -> StatusFlags {
        StatusFlags {
            new: false,
            modified: false,
            deleted: false,
            renamed: false,
            push: false,
            pull: false,
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut add_flag = |add: bool, icon: char| {
            if add {
                s.push(icon)
            } else {
                s.push('-')
            }
        };
        add_flag(self.new, 'N');
        add_flag(self.modified, 'M');
        add_flag(self.deleted, 'D');
        add_flag(self.renamed, 'R');
        add_flag(self.push, 'P');
        add_flag(self.pull, 'U');
        s
    }
}

fn statuses_to_string(statuses: Statuses<'_>) -> String {
    let mut flags = StatusFlags::new();

    for entry in statuses.iter() {
        let status = entry.status();
        if status.is_index_new() || status.is_wt_new() {
            flags.new = true;
        }
        if status.is_index_modified() || status.is_wt_modified() {
            flags.modified = true;
        }
        if status.is_index_deleted() || status.is_wt_deleted() {
            flags.deleted = true;
        }
        if status.is_index_renamed() || status.is_wt_renamed() {
            flags.renamed = true;
        }
    }
    flags.to_string()
}

pub fn explore_path(path: &Path, repositories: &mut Vec<Repository>) {
    if let Ok(repo) = Repository::open(path) {
        repositories.push(repo);
    } else {
        for child in path.read_dir().expect("read_dir call failed") {
            if let Ok(child) = child {
                if child.file_type().expect("File type failed").is_dir() {
                    explore_path(&child.path(), repositories);
                }
            }
        }
    }
}

pub fn print_statuses(statuses: Vec<RepositoryStatus>, url: bool) {
    let max_repo_name_length = statuses.iter().map(|x| x.name.len()).max().unwrap_or(0);
    for status in statuses.iter() {
        print!(
            "{:<width$} {}",
            status.name,
            status.status_flags,
            width = max_repo_name_length
        );
        if url {
            println!(" {}", status.origin_url);
        } else {
            println!();
        }
    }
}
