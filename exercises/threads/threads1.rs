// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.
// This program should wait until all the spawned threads have finished before exiting.


use std::thread;
use std::time::Duration;


fn main() {

    let mut handles  = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant :: now
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut completed_threads = 0;
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        let result = handle.join().unwrap();
        results.push(result);
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
    
}