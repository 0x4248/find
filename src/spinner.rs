/* Rust Spinner (spinner.rs)
 * A simple spinner for Rust
 */
pub mod spinner {
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    
    pub fn spinner_cleanup(){
        print!("\r");
        io::stdout().flush().unwrap();
    }
    
    pub fn start_spinner(frames: Vec<String>, message: String, stop_spinner: Arc<Mutex<bool>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut frame_index = 0;

            while !*stop_spinner.lock().unwrap() {
                print!("\r{} {}", message, frames[frame_index]);
                std::io::stdout().flush().unwrap();
                frame_index = (frame_index + 1) % frames.len();
                thread::sleep(Duration::from_millis(100));
            }
            io::stdout().flush().unwrap();
        })
    }
}