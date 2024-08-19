fn main() {
    println!("Lock-Free In-Memory Storage Example");

    let storage = lock_free_in_memory_storage::LockFreeStorage::new(10);
    storage.update(0, 42);
    println!("Updated value at index 0: {}", storage.read(0));
}
