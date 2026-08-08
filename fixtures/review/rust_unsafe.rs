use std::sync::Arc;
use std::thread;

fn main() {
    let mut data = Arc::new(vec![1, 2, 3]);

    let cloned = Arc::clone(&data);
    let handle = thread::spawn(move || {
        cloned.push(4);
    });

    data.push(5);

    handle.join().unwrap();
}
