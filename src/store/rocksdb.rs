use crate::error::Result;
use crate::store::{Database, DbBatch};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Basic Rocks db implementation.
/// Maybe add columns for key prefixes?
pub struct RocksDB {
    db: Arc<rocksdb::DB>,
    dbpath: PathBuf,
}

impl RocksDB {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let opts = RocksDB::db_options();
        let inner = rocksdb::DB::open(&opts.into(), &path)?;

        let mut path_buf = PathBuf::new();
        path_buf.push(path);

        Ok(Self {
            db: Arc::new(inner),
            dbpath: path_buf,
        })
    }

    fn db_options() -> rocksdb::Options {
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        opts.increase_parallelism(num_cpus::get() as i32);
        opts
    }

    pub fn destroy(self) -> Result<()> {
        let opts = RocksDB::db_options();
        drop(self.db);
        rocksdb::DB::destroy(&opts, &self.dbpath)?;
        Ok(())
    }
}

impl Database for RocksDB {
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
        match self.db.get_pinned(key) {
            Ok(value) => value.map(|v| v.to_vec()),
            Err(e) => panic!(e),
        }
    }

    // Commit batch to the db
    fn write_batch(&mut self, batch: DbBatch) -> Result<()> {
        let mut dbbatch = rocksdb::WriteBatch::default();
        for (k, v) in batch {
            dbbatch.put(k, v);
        }
        let mut opts = rocksdb::WriteOptions::default();
        opts.set_sync(false);
        self.db.write_opt(dbbatch, &opts)?;
        Ok(())
    }

    // Return an iterator over the nodes in the database
    fn iter<'a>(
        &'a self,
        start: &'a [u8],
        end: &'a [u8],
    ) -> Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + 'a> {
        Box::new(
            self.db
                .iterator(rocksdb::IteratorMode::From(
                    start,
                    rocksdb::Direction::Forward,
                ))
                .filter_map(move |(k, v)| {
                    if k[..] >= start[..] && k[..] <= end[..] {
                        Some((k, v))
                    } else {
                        None
                    }
                }),
        )
    }
}
