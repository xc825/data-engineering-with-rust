use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..20 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {} started", i);
            let mut data = data.lock().unwrap();
            println!("Thread {} got lock", i);
            println!("Thread {} data[{}]:{} -> {}", i, i%3, data[i%3], &data[i%3]+1);
            data[&i%3] += 1;
            sleep(Duration::from_millis(10));
        });
        println!("Thread {} spawned", i);
        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        handle.join().unwrap();
        println!("Thread {} finished", i);
    }

    println!("{:?}", data);

}
