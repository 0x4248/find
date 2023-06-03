/* Find (main.rs)
 * A file finder made in rust
 * Github: https://www.github.com/awesomelewis2007/find
 * Licence: GNU General Public License v3.0
 * By: Lewis Evans
*/

use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use termion;
mod dir;
mod spinner;
use spinner::spinner::spinner_cleanup;
use spinner::spinner::start_spinner;

const VERSION: &str = "1.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "-h" {
        println!("Usage: {} <filename> <path>", args[0]);
        println!("Options:");
        println!("\t-h\tShow this help message");
        println!("\t-v\tShow version");
        println!("\t-q\tQuiet mode");
        return;
    } else if args[1] == "-v" {
        println!("Find version: {}", VERSION);
        return;
    }

    let mut quiet = false;
    if args.len() == 2 && args[1] == "-q" {
        quiet = true;
    }

    if args.len() < 2 {
        println!("Usage: {} <filename> <path>", args[0]);
        println!("For help use the help option using: {} -h", args[0]);
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
    let spinner_thread = start_spinner(
        vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        "Searching for file".to_string(),
        100,
        stop_spinner.clone(),
    );

    let found_files = dir::search_files(&path, filename, &quiet);

    *stop_spinner.lock().unwrap() = true;
    spinner_thread.join().unwrap();
    spinner_cleanup();

    if found_files.is_empty() {
        if !quiet {
            println!(
                "{} Error{}: No files were found",
                termion::color::Fg(termion::color::Red),
                termion::style::Reset
            );
        }
    } else {
        for path in found_files {
            let message = if quiet {
                path.display().to_string()
            } else {
                format!(
                    "{}Found{}: {}",
                    termion::color::Fg(termion::color::Green),
                    termion::style::Reset,
                    path.display()
                )
            };
            println!("{}", message);
        }
    }
}
