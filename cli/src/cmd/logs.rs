use anyhow::{Result, Context};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Duration;
use std::thread;
use reverse_ssh_core::config::paths;

pub async fn run(id: String, follow: bool) -> Result<()> {
    let log_path = paths::logs_dir()?.join(format!("{}.log", id));
    
    if !log_path.exists() {
        println!("No logs found for profile '{}'.", id);
        return Ok(());
    }

    if follow {
        // Simple tail -f implementation
        let file = File::open(&log_path).context("Failed to open log file")?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(0) => {
                    // Wait for more data
                    thread::sleep(Duration::from_millis(500));
                }
                Ok(_) => {
                    print!("{}", line);
                    line.clear();
                }
                Err(e) => return Err(e.into()),
            }
        }
    } else {
        let content = std::fs::read_to_string(&log_path).context("Failed to read log file")?;
        print!("{}", content);
    }

    Ok(())
}
