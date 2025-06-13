use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

fn write_heavy_comparison() {
    const NUM_WRITERS: usize = 20;
    const WRITES_PER_THREAD: usize = 10_000;
    
    // Mutex benchmark
    let mutex_data = Arc::new(Mutex::new(0i64));
    let mem_size = std::mem::size_of::<Arc<Mutex<i64>>>();
    println!("Mutex size: {} bytes", mem_size);

    let start = Instant::now();
    
    let handles: Vec<_> = (0..NUM_WRITERS).map(|_| {
        let data = Arc::clone(&mutex_data);
        thread::spawn(move || {
            for _ in 0..WRITES_PER_THREAD {
                let mut guard = data.lock().unwrap();
                *guard += 1;
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_time = start.elapsed();
    println!("Mutex write-heavy: {:?}", mutex_time);
    
    // RwLock benchmark
    let rwlock_data = Arc::new(RwLock::new(0i64));
    let mem_size = std::mem::size_of::<Arc<RwLock<i64>>>();
    println!("RwLock size: {} bytes", mem_size);

    let start = Instant::now();
    
    let handles: Vec<_> = (0..NUM_WRITERS).map(|_| {
        let data = Arc::clone(&rwlock_data);
        thread::spawn(move || {
            for _ in 0..WRITES_PER_THREAD {
                let mut guard = data.write().unwrap();
                *guard += 1;
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let rwlock_time = start.elapsed();
    println!("RwLock write-heavy: {:?}", rwlock_time);
    
    // Mutex is typically 10-30% faster for write-heavy workloads
}

fn main() {
    write_heavy_comparison();
}