use std::cmp::Ordering;

use crate::error::Result;
use crate::node::{Link, Node};
use crate::proof;
use crate::sparse_tree::{SparseTree, TreeBatch};
use crate::store::Database;
use std::sync::Arc;

const ROOT_KEY_KEY: [u8; 6] = *b"\00root";

/// A handle to a Merkle key/value store backed by RocksDB.
pub struct Merk {
    pub tree: Option<Box<SparseTree>>,
    db: Arc<dyn Database>,
}

impl Merk {
    pub fn new(db: Arc<dyn Database>) -> Result<Merk> {
        let tree = match db.get(&ROOT_KEY_KEY) {
            Some(root_key) => {
                let root_node = get_node(db.as_ref(), &root_key)?;
                Some(Box::new(SparseTree::new(root_node)))
            }
            None => None,
        };
        Ok(Merk { tree, db })
    }

    pub fn get(&self, key: &[u8]) -> Result<Vec<u8>> {
        let node = get_node(self.db.as_ref(), key)?;
        Ok(node.value)
    }

    pub fn apply(&mut self, batch: &mut TreeBatch) -> Result<()> {
        let db = &*self.db;
        let get_node = |link: &Link| -> Result<Node> { get_node(db, &link.key) };

        // sort batch and ensure there are no duplicate keys
        let mut duplicate = false;
        batch.sort_by(|a, b| {
            let cmp = a.0.cmp(&b.0);
            if let Ordering::Equal = cmp {
                duplicate = true;
            }
            cmp
        });
        if duplicate {
            bail!("Batch must not have duplicate keys");
        }

        // apply tree operations, setting resulting root node in self.tree
        SparseTree::apply(&mut self.tree, &get_node, batch)?;

        // commit changes to db
        self.commit()
    }

    pub fn apply_unchecked(&mut self, batch: &TreeBatch) -> Result<()> {
        let db = &*self.db;
        let get_node = |link: &Link| -> Result<Node> { get_node(db, &link.key) };

        // apply tree operations, setting resulting root node in self.tree
        SparseTree::apply(&mut self.tree, &get_node, batch)?;

        // commit changes to db
        self.commit()
    }

    fn commit(&mut self) -> Result<()> {
        if let Some(tree) = &mut self.tree {
            let modified = tree.modified()?;
            self.db.write_batch(modified)?;
            self.db.put(&ROOT_KEY_KEY, tree.key.clone())?;
        } else {
            self.db.delete(&ROOT_KEY_KEY)?;
        }
        if let Some(tree) = &mut self.tree {
            tree.prune();
        }
        Ok(())
    }

    pub fn map_range<F: FnMut(Node)>(&self, start: &[u8], end: &[u8], f: &mut F) -> Result<()> {
        for (key, value) in self.db.iter(start, end) {
            let node = Node::decode(&key, &value)?;
            f(node);
        }
        Ok(())
    }

    pub fn map_branch<F: FnMut(&Node)>(&mut self, key: &[u8], f: &mut F) -> Result<()> {
        let tree_mut = self.tree.as_mut().map(|b| b.as_mut());

        let db = &*self.db;
        let mut get_node = |link: &Link| -> Result<Node> { get_node(db, &link.key) };

        SparseTree::map_branch(tree_mut, &mut get_node, key, f)
    }

    #[inline]
    pub fn proof(&mut self, start: &[u8], end: &[u8]) -> Result<Vec<proof::Op>> {
        proof::create(self, start, end)
    }
}

fn get_node<'a>(db: &'a dyn Database, key: &[u8]) -> Result<Node> {
    match db.get(key) {
        Some(bytes) => Node::decode(key, &bytes),
        None => bail!("key not found: '{:?}'", key),
    }
}

// fn concat(a: &[u8], b: &[u8]) -> Vec<u8> {
//     let mut result = Vec::with_capacity(a.len() + b.len());
//     result.extend_from_slice(a);
//     result.extend_from_slice(b);
//     result
// }
