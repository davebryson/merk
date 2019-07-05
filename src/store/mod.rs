use crate::error::Result;

/// A batch of entries
pub type DbBatch<'a> = Vec<(&'a [u8], Vec<u8>)>;

/// Trait for implementing some form of KV storage for merk
/// fn are based on db usage in the original merk implementation
pub trait Database: Send + Sync {
    fn put(&mut self, key: &[u8], value: Vec<u8>) -> Result<()>;

    fn delete(&mut self, key: &[u8]) -> Result<()>;

    /// Returns the bytes of  *node* for a given key. The consumer (merk) is
    /// responsible for decoding it
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Commits a DbBatch to the underlying storage
    /// Maybe add: fn flush() ?
    fn write_batch(&mut self, batch: DbBatch) -> Result<()>;

    /// Return an iterator over the nodes in the database in the range [start, end].
    /// Again the consumer of the iterator is responsible for decoding to Node.
    fn iter<'a>(
        &'a self,
        start: &'a [u8],
        end: &'a [u8],
    ) -> Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + 'a>;
}

pub mod memorydb;
pub mod rocksdb;
pub mod temporarydb;
