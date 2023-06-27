/* Rust Spinner (spinner.rs)
 * A simple spinner for Rust
 * Github: https://www.github.com/lewisevans2007/rust_spinner
 * Licence: GNU General Public License v3.0
 * By: Lewis Evans
*/

pub mod spinner {
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    /**
     * Clean up the spinner from the terminal
     */
    pub fn spinner_cleanup() {
        print!("\r");
        io::stdout().flush().unwrap();
    }

    /**
     * Spinner function
     */
    pub fn start_spinner(
        frames: Vec<String>,
        message: String,
        speed: u64,
        stop_spinner: Arc<Mutex<bool>>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut frame_index = 0;

            while !*stop_spinner.lock().unwrap() {
                print!("\r{} {}", message, frames[frame_index]);
                std::io::stdout().flush().unwrap();
                frame_index = (frame_index + 1) % frames.len();
                thread::sleep(Duration::from_millis(speed));
            }
            io::stdout().flush().unwrap();
        })
    }
}
