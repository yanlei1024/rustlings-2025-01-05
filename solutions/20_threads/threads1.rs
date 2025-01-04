// 这个程序会生成(spawn)多个线程，每个线程至少运行250毫秒，
// 并且每个线程都会返回其完成任务所花费的时间。
// 该程序应当等待所有已生成的线程都执行完毕，然后将它们的返回值收集到一个动态数组中。

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // 将所有线程的结果收集到 `results` 动态数组中。
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
