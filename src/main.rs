/* Find (main.rs)
 * A file finder made in rust
 * Github: https://www.github.com/awesomelewis2007/find
*/

use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod dir;

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
    let spinner_thread = start_spinner(stop_spinner.clone());

    let found_files = dir::search_files(&path, filename);

    *stop_spinner.lock().unwrap() = true;
    spinner_thread.join().unwrap();
    spinner_cleanup();

    if found_files.is_empty() {
        println!("No files found");
    } else {
        for path in found_files {
            println!("{}", path.display());
        }
    }
}

fn spinner_cleanup(){
    print!("\r");
    io::stdout().flush().unwrap();
}

fn start_spinner(stop_spinner: Arc<Mutex<bool>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let spinner_chars = vec!["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"];
        let mut spinner_index = 0;

        while !*stop_spinner.lock().unwrap() {
            print!("\rSearching {}", spinner_chars[spinner_index]);
            io::stdout().flush().unwrap();
            spinner_index = (spinner_index + 1) % spinner_chars.len();
            thread::sleep(Duration::from_millis(100));
        }
        io::stdout().flush().unwrap();
    })
}
