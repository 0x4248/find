use std::fs;
use std::path::{Path, PathBuf};
use termion;

pub fn search_files(dir: &Path, filename: &str) -> Vec<PathBuf> {
    let mut found_files = Vec::new();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_file() && path.file_name().unwrap() == filename {
                        found_files.push(path);
                    } else if path.is_dir() {
                        found_files.extend(search_files(&path, filename));
                    }
                }
            }
        }
        Err(_) => {
            println!(
                "{}Error{}: couldn't read {}",
                termion::color::Fg(termion::color::Red),
                termion::color::Fg(termion::color::Reset),
                dir.display()
            );
    
        }
    }
    found_files
}
