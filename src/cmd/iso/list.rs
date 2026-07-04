use std::ffi::OsStr;

use async_std::{fs, path::Path};
use gc_gcm::DirEntry;
use line_ending::LineEnding;
use regex::Regex;
use termtree::Tree;

use crate::gcm::Gcm;

pub async fn run(
    path: impl AsRef<Path>,
    filter: Option<&str>,
    regex: bool,
    insensitive: bool,
    pretty: bool,
) -> anyhow::Result<()> {
    let Some(filename) = path.as_ref().file_name().map(|f| f.to_os_string()) else {
        panic!("Iso path is not a file");
    };

    let data = fs::read(path).await?;
    let gcm = Gcm::new(data)?;

    let iter = gcm.0.filesystem.iter_root();

    let Some(filter) = filter else {
        print_files(iter, pretty, filename);
        return Ok(());
    };

    if regex {
        let re = Regex::new(filter)?;
        let iter = iter.filter(|e| re.is_match(e.entry_name()));
        print_files(iter, pretty, filename);
    } else {
        if insensitive {
            let iter = iter.filter(|e| {
                e.entry_name()
                    .to_lowercase()
                    .contains(&filter.to_lowercase())
            });
            print_files(iter, pretty, filename);
        } else {
            let iter = iter.filter(|e| e.entry_name().contains(filter));
            print_files(iter, pretty, filename);
        }
    }

    Ok(())
}

fn print_files<'d>(
    iter: impl Iterator<Item = DirEntry<'d>>,
    pretty: bool,
    filename: impl AsRef<OsStr>,
) {
    if pretty {
        let prefix = format!("/{}", filename.as_ref().to_string_lossy());
        let tree = collect_recursive_pretty(prefix, iter);
        println!("{tree}");
    } else {
        let files = collect_recursive(String::new(), iter);
        let line_sep = LineEnding::from_current_platform();
        let output = files.join(line_sep.as_str());
        println!("{output}");
    }
}

fn collect_recursive<'d>(prefix: String, iter: impl Iterator<Item = DirEntry<'d>>) -> Vec<String> {
    let mut names = Vec::new();

    for entry in iter {
        let name = format!("{prefix}/{}", entry.entry_name());
        names.push(name.clone());

        if let Some(child) = entry.iter_dir() {
            names.extend(collect_recursive(name, child));
        }
    }

    names
}

fn collect_recursive_pretty<'d>(
    prefix: String,
    iter: impl Iterator<Item = DirEntry<'d>>,
) -> Tree<String> {
    let mut names = Tree::new(prefix);

    for entry in iter {
        if entry.is_file() {
            names.push(entry.entry_name().to_string());
        }

        if let Some(child) = entry.iter_dir() {
            names.push(collect_recursive_pretty(
                entry.entry_name().to_string(),
                child,
            ));
        }
    }

    names
}
