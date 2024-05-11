use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// incr, decr, snapshot,
#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn incr(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut d = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("error: {:?}", e))?;
        let c = d.entry(key.into()).or_insert(0);
        *c += 1;

        anyhow::Ok(())
    }

    pub fn decr(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut d = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("error: {:?}", e))?;
        let c = d.entry(key.into()).or_insert(0);
        *c -= 1;

        anyhow::Ok(())
    }

    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, i64>> {
        anyhow::Ok(
            self.data
                .lock()
                .map_err(|e| anyhow::anyhow!("{}", e))?
                .clone(),
        )
    }
}
