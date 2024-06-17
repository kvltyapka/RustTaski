use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn min_time_to_collect_hay(k: u32, t: Vec<u32>) -> u32 {
    let t = Arc::new(Mutex::new(t));
    let mut handles = vec![];

    for i in 0..k {
        let t = Arc::clone(&t);
        let handle = thread::spawn(move || {
            let mut sum = 0;
            loop {
                let task = {
                    let mut t = t.lock().unwrap();
                    t.pop()
                };
                match task {
                    Some(time) => {
                        println!(
                            "Worker {} is taking a task that requires {} seconds.",
                            i + 1,
                            time
                        );
                        thread::sleep(Duration::from_secs(time.into()));
                        sum += time;
                    }
                    None => break,
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_worker_single_task() {
        let k = 1;
        let t = vec![5];
        assert_eq!(min_time_to_collect_hay(k, t), 5);
    }

    #[test]
    fn test_single_worker_multiple_tasks() {
        let k = 1;
        let t = vec![2, 3, 4];
        assert_eq!(min_time_to_collect_hay(k, t), 9);
    }

    #[test]
    fn test_multiple_workers_single_task_each() {
        let k = 3;
        let t = vec![2, 3, 4];
        assert_eq!(min_time_to_collect_hay(k, t), 4);
    }

    #[test]
    fn test_multiple_workers_uneven_tasks() {
        let k = 2;
        let t = vec![5, 5, 10];
        assert_eq!(min_time_to_collect_hay(k, t), 10);
    }

    #[test]
    fn test_no_tasks() {
        let k = 2;
        let t = vec![];
        // With no tasks, the time to collect hay is 0, regardless of the number of workers.
        assert_eq!(min_time_to_collect_hay(k, t), 0);
    }
}
