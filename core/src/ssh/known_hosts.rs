use std::path::Path;

use crate::error::{CoreError, Result};

/// Entry in a known_hosts file
#[derive(Debug, Clone)]
pub struct KnownHostEntry {
    /// Hostname or IP (may be hashed)
    pub host: String,
    /// Key type (e.g., ssh-rsa, ssh-ed25519)
    pub key_type: String,
    /// Base64-encoded public key
    pub key: String,
    /// Optional comment
    pub comment: Option<String>,
}

impl KnownHostEntry {
    /// Parse a line from known_hosts file
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        Some(Self {
            host: parts[0].to_string(),
            key_type: parts[1].to_string(),
            key: parts[2].to_string(),
            comment: parts.get(3).map(|s| s.to_string()),
        })
    }

    /// Format as known_hosts line
    pub fn to_line(&self) -> String {
        match &self.comment {
            Some(comment) => format!("{} {} {} {}", self.host, self.key_type, self.key, comment),
            None => format!("{} {} {}", self.host, self.key_type, self.key),
        }
    }
}

/// Manager for app-specific known_hosts file
pub struct KnownHostsManager {
    path: std::path::PathBuf,
    entries: Vec<KnownHostEntry>,
}

impl KnownHostsManager {
    /// Create a new manager for the given known_hosts file
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            entries: Vec::new(),
        }
    }

    /// Load entries from the file
    pub fn load(&mut self) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.path)
            .map_err(|e| CoreError::StorageAccess(format!("Failed to read known_hosts: {}", e)))?;

        self.entries.clear();
        for line in content.lines() {
            if let Some(entry) = KnownHostEntry::parse(line) {
                self.entries.push(entry);
            }
        }

        Ok(())
    }

    /// Save entries to the file
    pub fn save(&self) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CoreError::StorageAccess(format!("Failed to create directory: {}", e)))?;
        }

        let content: String = self.entries
            .iter()
            .map(|e| e.to_line())
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(&self.path, content)
            .map_err(|e| CoreError::StorageAccess(format!("Failed to write known_hosts: {}", e)))?;

        Ok(())
    }

    /// Add an entry (replaces existing for same host)
    pub fn add(&mut self, entry: KnownHostEntry) {
        // Remove existing entry for this host
        self.entries.retain(|e| e.host != entry.host);
        self.entries.push(entry);
    }

    /// Remove entries for a host
    pub fn remove(&mut self, host: &str) {
        self.entries.retain(|e| e.host != host);
    }

    /// Check if a host is known
    pub fn is_known(&self, host: &str) -> bool {
        self.entries.iter().any(|e| e.host == host)
    }

    /// Get entry for a host
    pub fn get(&self, host: &str) -> Option<&KnownHostEntry> {
        self.entries.iter().find(|e| e.host == host)
    }

    /// Get all entries
    pub fn entries(&self) -> &[KnownHostEntry] {
        &self.entries
    }

    /// Get the path to the known_hosts file
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_known_host_entry() {
        let line = "example.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... user@host";
        let entry = KnownHostEntry::parse(line).unwrap();
        
        assert_eq!(entry.host, "example.com");
        assert_eq!(entry.key_type, "ssh-ed25519");
        assert!(entry.key.starts_with("AAAAC3"));
        assert_eq!(entry.comment, Some("user@host".to_string()));
    }

    #[test]
    fn test_parse_empty_line() {
        assert!(KnownHostEntry::parse("").is_none());
        assert!(KnownHostEntry::parse("# comment").is_none());
    }

    #[test]
    fn test_entry_to_line() {
        let entry = KnownHostEntry {
            host: "example.com".to_string(),
            key_type: "ssh-ed25519".to_string(),
            key: "AAAAC3...".to_string(),
            comment: None,
        };
        
        assert_eq!(entry.to_line(), "example.com ssh-ed25519 AAAAC3...");
    }
}
