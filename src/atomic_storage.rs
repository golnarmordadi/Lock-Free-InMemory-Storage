use std::sync::atomic::{AtomicUsize, Ordering};

pub struct LockFreeStorage {
    data: Vec<AtomicUsize>,
}

impl LockFreeStorage {
    // Initialize the storage with a given size
    pub fn new(size: usize) -> Self {
        let data = (0..size).map(|_| AtomicUsize::new(0)).collect();
        LockFreeStorage { data }
    }

    // Update a value in the storage using atomic store
    pub fn update(&self, index: usize, value: usize) {
        self.data[index].store(value, Ordering::SeqCst);
    }

    // Read a value from the storage using atomic load
    pub fn read(&self, index: usize) -> usize {
        self.data[index].load(Ordering::SeqCst)
    }

    // Compare and swap operation for atomic updates
    pub fn update_cas(&self, index: usize, old_value: usize, new_value: usize) -> bool {
        match self.data[index].compare_exchange(
            old_value,         // Expected old value
            new_value,         // Desired new value
            Ordering::SeqCst,  // Memory ordering if successful
            Ordering::SeqCst   // Memory ordering if unsuccessful
        ) {
            Ok(_) => true,     // CAS succeeded
            Err(_) => false,   // CAS failed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_free_storage() {
        let storage = LockFreeStorage::new(10);
        storage.update(0, 42);
        assert_eq!(storage.read(0), 42);
        assert!(storage.update_cas(0, 42, 100));
        assert_eq!(storage.read(0), 100);
    }
}
