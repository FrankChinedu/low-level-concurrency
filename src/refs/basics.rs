use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn run() {
    let queue = Mutex::new(VecDeque::new());
    thread::scope(|s| {
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });
        for i in 0..10 {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    })
}
