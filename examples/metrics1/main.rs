use std::{thread, time::Duration};

use anyhow::anyhow;
use rand::{self, Rng};

use concurrency::atomic_map::AtomicMetrics;

pub fn main() -> anyhow::Result<()> {
    let keys = ["request1", "page-2"];
    let metric = AtomicMetrics::new(&keys);

    for _ in 0..2 {
        task_worker(metric.clone())?;
    }

    for _ in 0..3 {
        request_worker(metric.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metric);
    }
}

fn task_worker(metric: AtomicMetrics) -> anyhow::Result<()> {
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(1000));

        let _ = metric.incr("request1").map_err(|e| anyhow!("error: {}", e));
    });

    anyhow::Ok(())
}

fn request_worker(metric: AtomicMetrics) -> anyhow::Result<()> {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(Duration::from_millis(rng.gen_range(50..500)));
        let _ = metric.incr("page-2").map_err(|e| anyhow!("error: {}", e));
    });

    anyhow::Ok(())
}
