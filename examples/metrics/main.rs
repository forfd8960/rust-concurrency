use std::thread;
use std::time::Duration;

use concurrency::metrics::Metrics;
use rand::Rng;

pub fn main() -> anyhow::Result<()> {
    let metric = Metrics::new();

    for idx in 0..2 {
        task_worker(idx, metric.clone());
    }

    for _ in 0..3 {
        request_worker(metric.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metric);
    }

    // anyhow::Ok(())
}

fn task_worker(idx: usize, metric: Metrics) {
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(1000));

        if let Err(e) = metric.incr(format!("request-{}", idx)) {
            eprintln!("error: {}", e);
        };
    });
}

fn request_worker(metric: Metrics) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..500)));

            let page = rng.gen_range(1..10);
            metric.incr(format!("page-{}", page))?;
        }

        #[allow(unreachable_code)]
        anyhow::Ok(())
    });

    anyhow::Ok(())
}
