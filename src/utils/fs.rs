use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use walkdir::{DirEntry, WalkDir};

pub fn create_dir_all(path: &str) -> io::Result<()> {
    if path.is_empty() {
        return Ok(());
    }

    if !Path::new(path).exists() {
        fs::create_dir_all(path)
    } else {
        Ok(())
    }
}

pub fn create_file_if_not_exists(path: &str) -> io::Result<()> {
    let p = Path::new(path);
    if p.exists() {
        return Ok(());
    }

    if let Some(p) = p.parent() {
        create_dir_all(p.to_str().unwrap())?
    }

    let _ = File::create(path)?;

    Ok(())
}

pub fn list_all_files(path: &str) -> io::Result<Vec<PathBuf>> {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
    }

    let mut res: Vec<PathBuf> = vec![];

    for entry in WalkDir::new(path).into_iter().filter_entry(|e| !is_hidden(e)) {
        let entry = entry?;
        if entry.file_type().is_dir() || entry.path_is_symlink() {
            // skip dir or symlink
            continue;
        }
        res.push(entry.into_path());
    }

    Ok(res)
}

pub fn hard_link_file<P: AsRef<Path>>(original: P, link: &str) -> io::Result<()> {
    let p = Path::new(link);
    if let Some(p) = p.parent() {
        create_dir_all(p.to_str().unwrap())?
    }

    fs::hard_link(original, link)
}
