use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Вычисляет минимальное время, необходимое для сбора сена всеми работниками.
///
/// # Аргументы
///
/// * `k` - Количество работников.
/// * `t` - Вектор, содержащий время, необходимое для сбора каждого участка сена.
///
/// # Возвращаемое значение
///
/// Возвращает `u32`, минимальное время, за которое все работники смогут собрать сено.
///
/// # Особенности реализации
///
/// Функция делит общее количество работы поровну между всеми работниками. Если деление не равномерное,
/// оставшуюся часть работы распределяет между работниками по одному дополнительному элементу, пока не закончится.
/// Использует многопоточность для параллельного выполнения работы и `Mutex` для синхронизации доступа к общему ресурсу.
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

fn main() {
    let n = 3;
    let m = 3;
    let k = 3;
    let t: Vec<u32> = (1..=n * m).map(|x| x as u32).collect();

    let min_time = min_time_to_collect_hay(k, t);
    println!("Minimum time to collect all hay: {}", min_time);
}
