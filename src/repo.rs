use git2::{Repository, Statuses};
use std::path::Path;

pub struct RepositoryStatus {
    name: String,
    status_flags: StatusFlags,
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

    fn clean(&self) -> bool {
        !self.new && !self.modified && !self.deleted && !self.renamed && !self.push && !self.pull
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

fn statuses_to_string(statuses: Statuses<'_>) -> StatusFlags {
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
    flags
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

pub fn print_long(statuses: Vec<RepositoryStatus>, url: bool, show_clean: bool) {
    let max_repo_name_length = statuses.iter().map(|x| x.name.len()).max().unwrap_or(0);
    for status in statuses.iter() {
        if status.status_flags.clean() && !show_clean {
            continue;
        }
        print!(
            "{:<width$} {}",
            status.name,
            status.status_flags.to_string(),
            width = max_repo_name_length
        );
        if url {
            println!(" {}", status.origin_url);
        } else {
            println!();
        }
    }
}

struct Summary {
    clean: u32,
    dirty: u32,
    modified: u32,
    untracked: u32,
    unsynced: u32,
}

impl Summary {
    fn new() -> Summary {
        Summary {
            clean: 0,
            dirty: 0,
            modified: 0,
            untracked: 0,
            unsynced: 0,
        }
    }

    fn print(&self) {
        println!("{} clean repositories", self.clean);
        if self.dirty > 0 {
            println!("{} repositories in dirty state, of which:", self.dirty);
        }
        if self.modified > 0 {
            println!("\t{} repositories with modified files", self.modified);
        }
        if self.untracked > 0 {
            println!("\t{} repositories with untracked files", self.untracked);
        }
        if self.unsynced > 0 {
            println!("\t{} repositories with unsynced commits", self.unsynced);
        }
    }
}

pub fn print_summary(statuses: Vec<RepositoryStatus>) {
    let mut summary = Summary::new();
    for status in statuses.iter() {
        if status.status_flags.clean() {
            summary.clean += 1;
        } else {
            summary.dirty += 1;
        }
        if status.status_flags.modified {
            summary.modified += 1;
        }
        if status.status_flags.new {
            summary.untracked += 1;
        }
        // TODO: check if there are commits to push/pull
    }
    summary.print();
}
