use std::thread::{self, JoinHandle};

fn main() {
    println!("Thread Examples");
    let mut threads: Vec<JoinHandle<()>> = vec![];

    for i in 0..10 {
        let t1 = thread::spawn(move || {
            println!("thread_{} is running...", i);
        });

        threads.push(t1);
    }

    for t in threads {
        t.join().unwrap();
    }
}
