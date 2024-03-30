// metrics data structure
// 基本功能：inc/dec/snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>, // Arc<Mutex<HashMap<String, i64>>> => Arc<DashMap<String, i64>>
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
