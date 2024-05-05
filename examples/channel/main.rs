use std::time::Duration;
use std::{sync::mpsc, thread};

use anyhow::{anyhow, Ok, Result};

const NUM_PRODUCER: usize = 4;

#[derive(Debug)]
struct Message {
    idx: usize,
    value: i32,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCER {
        let tx = tx.clone();
        thread::spawn(move || {
            producer(i, tx);
        });
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("msg: {}, {}", msg.idx, msg.value);
        }
        println!("consumer exit");
    });

    consumer.join().map_err(|e| anyhow!("{:?}", e))?;
    Ok(())
}

fn producer(idx: usize, sender: mpsc::Sender<Message>) {
    loop {
        let value = rand::random::<i32>();
        sender.send(Message::new(idx, value)).unwrap();
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        if rand::random::<u8>() % 10 == 0 {
            println!("producer: {} exit", idx);
            break;
        }
    }
}

impl Message {
    pub fn new(idx: usize, value: i32) -> Self {
        Self { idx, value }
    }
}
