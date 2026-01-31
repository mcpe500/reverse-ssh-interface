use anyhow::{Result, Context};
use reverse_ssh_core::config::paths;
use std::fs;
use std::io::{BufRead, BufReader};

pub async fn run(session_id: Option<String>, follow: bool, lines: usize) -> Result<()> {
    let logs_dir = paths::logs_dir();

    if !logs_dir.exists() {
        println!("No logs directory found.");
        return Ok(());
    }

    if let Some(id) = session_id {
        // Show logs for specific session
        let log_file = logs_dir.join(format!("{}.log", id));
        if log_file.exists() {
            show_log_file(&log_file, lines, follow).await?;
        } else {
            println!("No logs found for session: {}", id);
        }
    } else {
        // List available log files
        let entries = fs::read_dir(&logs_dir)
            .context("Failed to read logs directory")?;

        let mut log_files: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "log").unwrap_or(false))
            .collect();

        if log_files.is_empty() {
            println!("No log files found.");
            return Ok(());
        }

        // Sort by modification time (newest first)
        log_files.sort_by(|a, b| {
            let time_a = a.metadata().and_then(|m| m.modified()).ok();
            let time_b = b.metadata().and_then(|m| m.modified()).ok();
            time_b.cmp(&time_a)
        });

        println!("Available log files:");
        for entry in log_files {
            let path = entry.path();
            let name = path.file_stem().unwrap_or_default().to_string_lossy();
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            println!("  {} ({} bytes)", name, size);
        }
        println!("\nUse 'rssh logs <session-id>' to view a specific log.");
    }

    Ok(())
}

async fn show_log_file(path: &std::path::Path, lines: usize, follow: bool) -> Result<()> {
    let file = fs::File::open(path)
        .context("Failed to open log file")?;
    let reader = BufReader::new(file);

    let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let start = all_lines.len().saturating_sub(lines);

    for line in all_lines.iter().skip(start) {
        println!("{}", line);
    }

    if follow {
        println!("--- Following log (Ctrl+C to stop) ---");
        // In a real implementation, we'd use notify or similar for file watching
        // For now, just poll the file
        use std::time::Duration;
        let mut last_pos = all_lines.len();

        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

            let file = fs::File::open(path)?;
            let reader = BufReader::new(file);
            let current_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

            if current_lines.len() > last_pos {
                for line in current_lines.iter().skip(last_pos) {
                    println!("{}", line);
                }
                last_pos = current_lines.len();
            }
        }
    }

    Ok(())
}
