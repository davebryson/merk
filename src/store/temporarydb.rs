use crate::error::Result;
use crate::store::rocksdb::RocksDB;
use crate::store::Database;

use tempfile::TempDir;

/// Temporary directory wrapper around RocksDB. TempDir automatically deletes
/// the underlying directory once TemoraryDB goes out of scope.
/// ** Unused but needed to keep the TempDir from dropping under certain circumstances
pub struct TemporaryDB {
    db: RocksDB,
    _tdir: TempDir, // **
}

impl TemporaryDB {
    pub fn new() -> Self {
        let dir = TempDir::new().unwrap();
        Self {
            db: RocksDB::open(&dir).unwrap(),
            _tdir: dir,
        }
    }
}

impl Database for TemporaryDB {
    fn put(&self, key: &[u8], value: Vec<u8>) -> Result<()> {
        self.db.put(key, value)
    }

    fn delete(&self, key: &[u8]) -> Result<()> {
        self.db.delete(key)
    }

    // Return a node for a given key
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key)
    }

    // Commit batch to the db
    fn write_batch<'a>(&self, batch: Vec<(&'a [u8], Vec<u8>)>) -> Result<()> {
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
