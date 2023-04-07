/* Find (main.rs)
 * A file finder made in rust
 * Github: https://www.github.com/awesomelewis2007/find
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

    if args.len() < 2 {
        println!("Usage: {} <filename> <path>", args[0]);
        return;
    }

    let filename = &args[1];
    let path = if args.len() == 3 {
        Path::new(&args[2])
    } else {
        Path::new(".")
    };

    let stop_spinner = Arc::new(Mutex::new(false));
    let spinner_thread = start_spinner(vec!["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"].iter().map(|s| s.to_string()).collect(), "Loading".to_string(), stop_spinner.clone());

    let found_files = dir::search_files(&path, filename);

    *stop_spinner.lock().unwrap() = true;
    spinner_thread.join().unwrap();
    spinner_cleanup();

    if found_files.is_empty() {
        println!("{} Error{}: No files found", termion::color::Fg(termion::color::Red), termion::style::Reset);
    } else {
        for path in found_files {
            println!("{}", path.display());
        }
    }
}
