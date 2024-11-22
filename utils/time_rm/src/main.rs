use std::io::Write;
use std::thread;
use std::time::Duration;
use chrono::Local;

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        let now = Local::now();
        print!("RM553632 | {} ", now.format("%d-%m-%Y %H:%M:%S"));
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}