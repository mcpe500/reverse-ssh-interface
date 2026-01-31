use std::time::Duration;

/// Exponential backoff calculator for reconnection attempts
#[derive(Debug, Clone)]
pub struct Backoff {
    /// Initial delay between attempts
    initial_delay: Duration,
    /// Maximum delay between attempts
    max_delay: Duration,
    /// Multiplier for each attempt
    multiplier: f64,
    /// Current attempt number
    attempt: u32,
    /// Maximum number of attempts (0 = unlimited)
    max_attempts: u32,
}

impl Backoff {
    /// Create a new backoff calculator with default settings
    pub fn new() -> Self {
        Self {
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(300), // 5 minutes max
            multiplier: 2.0,
            attempt: 0,
            max_attempts: 0,
        }
    }

    /// Set the initial delay
    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Set the maximum delay
    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Set the multiplier
    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// Set maximum attempts (0 = unlimited)
    pub fn with_max_attempts(mut self, max: u32) -> Self {
        self.max_attempts = max;
        self
    }

    /// Get the next delay and increment attempt counter
    /// Returns None if max attempts reached
    pub fn next_delay(&mut self) -> Option<Duration> {
        if self.max_attempts > 0 && self.attempt >= self.max_attempts {
            return None;
        }

        let delay = self.calculate_delay();
        self.attempt += 1;
        Some(delay)
    }

    /// Calculate delay for current attempt without incrementing
    pub fn calculate_delay(&self) -> Duration {
        let delay_secs = self.initial_delay.as_secs_f64() * self.multiplier.powi(self.attempt as i32);
        let delay = Duration::from_secs_f64(delay_secs);
        
        if delay > self.max_delay {
            self.max_delay
        } else {
            delay
        }
    }

    /// Reset the backoff counter
    pub fn reset(&mut self) {
        self.attempt = 0;
    }

    /// Get the current attempt number
    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    /// Check if max attempts reached
    pub fn is_exhausted(&self) -> bool {
        self.max_attempts > 0 && self.attempt >= self.max_attempts
    }
}

impl Default for Backoff {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backoff_delays() {
        let mut backoff = Backoff::new()
            .with_initial_delay(Duration::from_secs(1))
            .with_multiplier(2.0);

        // First delay: 1s
        assert_eq!(backoff.next_delay(), Some(Duration::from_secs(1)));
        // Second delay: 2s
        assert_eq!(backoff.next_delay(), Some(Duration::from_secs(2)));
        // Third delay: 4s
        assert_eq!(backoff.next_delay(), Some(Duration::from_secs(4)));
    }

    #[test]
    fn test_backoff_max_delay() {
        let mut backoff = Backoff::new()
            .with_initial_delay(Duration::from_secs(100))
            .with_max_delay(Duration::from_secs(200))
            .with_multiplier(2.0);

        // First delay: 100s
        assert_eq!(backoff.next_delay(), Some(Duration::from_secs(100)));
        // Second delay: capped at 200s
        assert_eq!(backoff.next_delay(), Some(Duration::from_secs(200)));
        // Third delay: still capped at 200s
        assert_eq!(backoff.next_delay(), Some(Duration::from_secs(200)));
    }

    #[test]
    fn test_backoff_max_attempts() {
        let mut backoff = Backoff::new()
            .with_max_attempts(3);

        assert!(backoff.next_delay().is_some());
        assert!(backoff.next_delay().is_some());
        assert!(backoff.next_delay().is_some());
        assert!(backoff.next_delay().is_none()); // Exhausted
        assert!(backoff.is_exhausted());
    }

    #[test]
    fn test_backoff_reset() {
        let mut backoff = Backoff::new()
            .with_max_attempts(2);

        backoff.next_delay();
        backoff.next_delay();
        assert!(backoff.is_exhausted());

        backoff.reset();
        assert!(!backoff.is_exhausted());
        assert_eq!(backoff.attempt(), 0);
    }
}
