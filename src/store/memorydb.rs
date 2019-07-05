use crate::error::Result;
use crate::store::{Database, DbBatch};

use std::collections::BTreeMap;
use std::sync::RwLock;

/// Memory backed Database
pub struct MemoryDb {
    map: RwLock<BTreeMap<Vec<u8>, Vec<u8>>>,
}

impl MemoryDb {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(BTreeMap::new()),
        }
    }
}

impl Database for MemoryDb {
    fn write_batch(&mut self, batch: DbBatch) -> Result<()> {
        let mut guard = self.map.write().expect("Lock");
        for (key, value) in batch {
            guard.insert(Vec::from(key), value);
        }
        Ok(())
    }

    fn put(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        let mut guard = self.map.write().expect("Lock");
        guard.insert(Vec::from(key), value);
        Ok(())
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        let mut guard = self.map.write().expect("Lock");
        guard.remove(key);
        Ok(())
    }

    // Return a node for a given key
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        match self.map.read() {
            Ok(map) => map.get(key).and_then(|v| Some(v.clone())),
            Err(_) => None,
        }
    }

    // Return an iterator over the nodes in the database from [start, end]
    fn iter<'a>(
        &'a self,
        start: &'a [u8],
        end: &'a [u8],
    ) -> Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + 'a> {
        Box::new(
            self.map
                .read()
                .unwrap()
                .clone()
                .into_iter()
                .filter_map(move |(k, v)| {
                    // Filter down to the inclusive range
                    if k[..] >= start[..] && k[..] <= end[..] {
                        Some((k.into_boxed_slice(), v.into_boxed_slice()))
                    } else {
                        None
                    }
                }),
        )
    }
}
