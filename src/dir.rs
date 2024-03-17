/* Find (dir.rs)
 * A file finder made in rust
 * Github: https://www.github.com/lewisevans2007/find
 * Licence: GNU General Public License v3.0
 * By: Lewis Evans
*/

use std::fs;
use std::path::{Path, PathBuf};
use termion;

/**
 * search_files - recursively search a directory for a file
 * @param dir: the directory to search
 * @param filename: the name of the file to search for
 * @param quiet: whether to print errors or not
 * @return a vector of paths to the found files
 */
pub fn search_files(dir: &Path, filename: &str, quiet: &bool) -> Vec<PathBuf> {
    let mut found_files = Vec::new();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_file() && path.file_name().unwrap() == filename {
                        found_files.push(path);
                    } else if path.is_dir() {
                        found_files.extend(search_files(&path, filename, quiet));
                    }
                }
            }
        }

        Err(_) => {
            if !quiet {
                println!(
                    "{}Error{}: couldn't read {}",
                    termion::color::Fg(termion::color::Red),
                    termion::color::Fg(termion::color::Reset),
                    dir.display()
                );
            }
        }
    }
    found_files
}
