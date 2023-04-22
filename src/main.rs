/* Find (main.rs)
 * A file finder made in rust
 * Github: https://www.github.com/awesomelewis2007/find
 * Licence: GPL-3.0
 * By: Lewis Evans
*/

use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use termion;
mod dir;
mod spinner;
use spinner::spinner::start_spinner;
use spinner::spinner::spinner_cleanup;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut quiet = false;
    if args.len() == 2 && args[1] == "-q" {
        quiet = true;
    }

    if args.len() < 2 {
        println!("Usage: {} <filename> <path>", args[0]);
        return;
    }
    let filename;
    if quiet {
        filename = &args[2];
    } else {
        filename = &args[1];
    }
    let path = if args.len() == 3 || (args.len() == 4 && args[1] == "-q") {
        if quiet {
            Path::new(&args[3])
        } else {
            Path::new(&args[2])
        }
    } else {
        Path::new(".")
    };

    let stop_spinner = Arc::new(Mutex::new(false));
    let spinner_thread = start_spinner(vec!["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"].iter().map(|s| s.to_string()).collect(), "Searching for file".to_string(), 100 , stop_spinner.clone());

    let found_files = dir::search_files(&path, filename, &quiet);

    *stop_spinner.lock().unwrap() = true;
    spinner_thread.join().unwrap();
    spinner_cleanup();

    if found_files.is_empty() {
        if !quiet {
            println!("{} Error{}: No files found", termion::color::Fg(termion::color::Red), termion::style::Reset);
        }
    } else {
        for path in found_files {
            if quiet {
                println!("{}", path.display());
            } else {
                println!("{}Found{}: {}", termion::color::Fg(termion::color::Green), termion::style::Reset, path.display());
            }
        }
    }
}
