# Lock-Free In-Memory Storage

## Overview

`lock_free_in_memory_storage` is a lock-free, thread-safe in-memory storage system implemented in Rust. The library provides concurrent access to in-memory data structures without the need for traditional locking mechanisms. Instead, it leverages atomic operations to ensure safe concurrent updates and reads.

## Key Features

1- Lock-Free In-Memory Storage:
A memory-efficient storage structure that supports concurrent updates and reads without locks.

2- Thread-Safe Operations:
Ensure that operations like inserts, updates, and reads are thread-safe using atomic operations and lock-free algorithms.

3- Linearizability:
Guarantee that every operation appears instantaneous and consistent to the external observer.

4- Efficient Handling of Contention:
Implement contention management techniques such as backoff or retries in high-contention scenarios.

## Technical Requirements

- Rust Version: 1.60+
- Concurrency Model: Multiple threads should be able to concurrently read and update the storage without needing to block or wait for each other.

## Target Audience

- Rust developers interested in concurrent programming and systems development.
- Anyone seeking to build high-performance, multi-threaded applications that require shared state without locks.

## Getting Started

### Prerequisites

- Rust (version 1.60 or higher). Install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

### Summary of Commands

- Build the project: ```bash cargo build```
- Run the tests: ```bash cargo test```
- Run the executable (if any): ```bash cargo run```

### File Summary

- atomic_storage.rs:
Contains the core implementation of the lock-free in-memory storage using atomic operations.
- lib.rs:
Serves as the entry point for the library and re-exports the lock-free storage.
- tests.rs:
Provides unit and integration tests to ensure the correct functionality of the system, especially under concurrent conditions.
- utils.rs:
Contains utility functions such as a backoff mechanism to deal with contention or retries.
