// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status_pair = Arc::new((Mutex::new(JobStatus { jobs_completed: 0 }), Condvar::new()));
    let status_pair2 = Arc::clone(&status_pair);
    thread::spawn(move || {
        for _ in 0..10 {
            let (lock, cvar) = &*status_pair2;
            thread::sleep(Duration::from_millis(250));
            let mut jobStatus = lock.lock().unwrap();
            (*jobStatus).jobs_completed += 1;
            cvar.notify_one()
        }
    });

    let (lock, cvar) = &*status_pair;
    let mut jobStatus = lock.lock().unwrap();
    while (*jobStatus).jobs_completed < 10 {
        println!("waiting... ");
        jobStatus = cvar.wait(jobStatus).unwrap();
        thread::sleep(Duration::from_millis(500))
    }

/*
    while status.lock().jobs_completed < 10 {
    }
    */
}
