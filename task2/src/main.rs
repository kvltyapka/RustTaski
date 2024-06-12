use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn min_time_to_collect_hay(k: u32, t: Vec<u32>) -> u32 {
    let t = Arc::new(Mutex::new(t));
    let mut handles = vec![];

    for _ in 0..k {
        let t = Arc::clone(&t);
        let handle = thread::spawn(move || {
            let mut sum = 0;
            while let Some(time) = {
                let mut t = t.lock().unwrap();
                t.pop()
            } {
                sum += time;
                thread::sleep(Duration::from_secs(time.into()));
            }
            sum
        });
        handles.push(handle);
    }

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .max()
        .unwrap()
}

fn main() {
    let n = 4;
    let m = 5;
    let k = 5;
    let t: Vec<u32> = (1..=n * m).map(|x| x as u32).collect();

    let min_time = min_time_to_collect_hay(k, t);
    println!("Minimum time to collect all hay: {}", min_time);
}
