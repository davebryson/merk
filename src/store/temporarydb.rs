use crate::error::Result;
use crate::store::rocksdb::RocksDB;
use crate::store::{Database, DbBatch};

use tempfile::TempDir;

pub struct TemporaryDB {
    db: RocksDB,
    tdir: TempDir,
}

impl TemporaryDB {
    pub fn new() -> Self {
        let dir = TempDir::new().unwrap();
        Self {
            db: RocksDB::open(&dir).unwrap(),
            tdir: dir,
        }
    }
}

impl Database for TemporaryDB {
    fn put(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        self.db.put(key, value);
        Ok(())
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.db.delete(key);
        Ok(())
    }

    // Return a node for a given key
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key)
    }

    // Commit batch to the db
    fn write_batch(&mut self, batch: DbBatch) -> Result<()> {
        self.db.write_batch(batch)
    }

    // Return an iterator over the nodes in the database in the range [start, end]
    fn iter<'a>(
        &'a self,
        start: &'a [u8],
        end: &'a [u8],
    ) -> Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + 'a> {
        self.db.iter(start, end)
    }
}
