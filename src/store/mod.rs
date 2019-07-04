// Trait for Database implementation

use crate::error::Result;

pub type DbBatch<'a> = Vec<(&'a [u8], Vec<u8>)>;

pub trait Database: Send + Sync {
    fn put(&mut self, key: &[u8], value: Vec<u8>) -> Result<()>;

    fn delete(&mut self, key: &[u8]) -> Result<()>;

    // Return a node for a given key
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    // Commit batch to the db
    fn write_batch(&mut self, batch: DbBatch) -> Result<()>;

    // Return an iterator over the nodes in the database in the range [start, end]
    fn iter<'a>(
        &'a self,
        start: &'a [u8],
        end: &'a [u8],
    ) -> Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + 'a>;
}

pub mod memorydb;
pub mod rocksdb;
pub mod temporarydb;
