use std::fmt;
use std::sync::Arc;

use dashmap::DashMap;

// incr, decr, snapshot,
#[derive(Debug, Clone)]
pub struct CMapMetrics {
    pub data: Arc<DashMap<String, i64>>,
}

impl Default for CMapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CMapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            let _ = writeln!(f, "{}: {}", entry.key(), entry.value());
        }

        std::result::Result::Ok(())
    }
}

impl CMapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn incr(&self, key: impl Into<String>) {
        let mut value = self.data.entry(key.into()).or_insert(0);
        *value += 1;
    }

    pub fn decr(&self, key: impl Into<String>) {
        let mut value = self.data.entry(key.into()).or_insert(0);
        *value -= 1;
    }
}
