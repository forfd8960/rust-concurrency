use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

use anyhow::anyhow;

#[derive(Debug)]
pub struct AtomicMetrics {
    pub data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AtomicMetrics {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl fmt::Display for AtomicMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in self.data.iter() {
            let _ = writeln!(f, "{}: {}", k, v.load(Ordering::Relaxed));
        }

        std::result::Result::Ok(())
    }
}

impl AtomicMetrics {
    pub fn new(metrics_names: &[&'static str]) -> Self {
        let data = metrics_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect::<HashMap<&'static str, AtomicI64>>();

        Self {
            data: Arc::new(data),
        }
    }

    pub fn incr(&self, key: &str) -> anyhow::Result<()> {
        let counter = self.data.get(key).ok_or_else(|| anyhow!("key not found"))?;
        counter.fetch_add(1, Ordering::Relaxed);
        anyhow::Ok(())
    }

    pub fn decr(&self, key: &str) -> anyhow::Result<()> {
        let counter = self.data.get(key).ok_or_else(|| anyhow!("key not found"))?;
        counter.fetch_add(-1, Ordering::Relaxed);
        anyhow::Ok(())
    }
}
