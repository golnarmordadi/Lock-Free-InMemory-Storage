
// Utility functions that could assist with common tasks across the library

/// Perform an exponential backoff strategy in case of high contention
pub fn backoff(attempts: u32) {
    let delay = std::time::Duration::from_micros(1 << attempts);
    std::thread::sleep(delay);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backoff() {
        // Test the backoff function with a few iterations
        for i in 0..5 {
            backoff(i);
        }
    }
}
