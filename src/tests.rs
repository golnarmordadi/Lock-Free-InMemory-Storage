
use std::sync::Arc;
use std::thread;
use crate::LockFreeStorage;

#[test]
fn test_concurrent_updates() {
    let storage = Arc::new(LockFreeStorage::new(1000));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let storage = Arc::clone(&storage);
            thread::spawn(move || {
                for i in 0..1000 {
                    storage.update(i, i + 1);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    for i in 0..1000 {
        assert_eq!(storage.read(i), 10);
    }
}

#[test]
fn test_cas() {
    let storage = LockFreeStorage::new(10);

    assert_eq!(storage.read(0), 0);
    assert!(storage.update_cas(0, 0, 123));
    assert_eq!(storage.read(0), 123);
    assert!(!storage.update_cas(0, 0, 456));  // Should fail since the value is no longer 0
}
