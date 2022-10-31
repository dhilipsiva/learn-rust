use std::thread;
use std::time::Duration;

pub fn ch_16_01() {
    let handle = thread::spawn(|| {
        for i in 1..15 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
