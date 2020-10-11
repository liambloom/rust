use std::thread;
use std::time::Duration;

fn main() {
    let spawned_max = 10;

    let handle = thread::spawn(move || {
        for i in 1..spawned_max {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // This works because spawned_max implements Copy
    // For most things, this would throw a compiler error, because spawned_max
    //  would have been moved
    drop(spawned_max);

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 
    handle.join().unwrap();
}
