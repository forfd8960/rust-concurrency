use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

// incr, decr, snapshot,
#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.read().unwrap();
        for (k, v) in data.iter() {
            let _ = writeln!(f, "{}: {}", k, v);
        }

        std::result::Result::Ok(())
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn incr(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut d = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!("error: {:?}", e))?;
        let c = d.entry(key.into()).or_insert(0);
        *c += 1;

        anyhow::Ok(())
    }

    pub fn decr(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut d = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!("error: {:?}", e))?;
        let c = d.entry(key.into()).or_insert(0);
        *c -= 1;

        anyhow::Ok(())
    }
}
