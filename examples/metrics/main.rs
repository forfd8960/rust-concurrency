use std::thread;
use std::time::Duration;

use concurrency::metrics::cmap::CMapMetrics;
use rand::Rng;

pub fn main() -> anyhow::Result<()> {
    let metric = CMapMetrics::new();

    for idx in 0..2 {
        task_worker(idx, metric.clone());
    }

    for _ in 0..3 {
        request_worker(metric.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metric);
    }

    // anyhow::Ok(())
}

fn task_worker(idx: usize, metric: CMapMetrics) {
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(1000));

        metric.incr(format!("request-{}", idx))
    });
}

fn request_worker(metric: CMapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..500)));

        let page = rng.gen_range(1..10);
        metric.incr(format!("page-{}", page));
    });
}
