use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn ch_16_02() {
    let (s, r) = mpsc::channel();
    thread::spawn(move || {
        for i in 0..15 {
            s.send(i).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
    for i in r {
        dbg!(i);
    }
}
