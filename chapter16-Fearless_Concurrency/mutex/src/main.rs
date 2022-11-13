use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 当循环时，counter所属权会被转移值多个线程中
    // 所以需要用到引用计数，管理多种所属权
    for _ in 0..10 {
        // 原子引用计数Arc<T>
        // 行为与Rc<T>一直，但是是并发安全的
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
