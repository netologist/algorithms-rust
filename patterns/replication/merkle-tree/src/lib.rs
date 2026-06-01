use hex::encode;
use sha2::{Digest, Sha256};

/// A Merkle tree built over byte-slice leaves.
///
/// Each leaf is the SHA-256 hash of its data.
/// Parent nodes are SHA-256 hashes of the concatenation of their two children.
/// When the number of nodes at a level is odd, the last node is duplicated.
///
/// The root hash summarises all data: any change to a single leaf produces a
/// completely different root, enabling efficient integrity verification.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: Option<String>,
    leaves: Vec<String>,
}

impl MerkleTree {
    /// Build a Merkle tree from a slice of byte payloads.
    ///
    /// Returns an empty tree (root = `None`) when `data` is empty.
    pub fn new(data: &[impl AsRef<[u8]>]) -> Self {
        let leaves: Vec<String> = data
            .iter()
            .map(|d| encode(Sha256::digest(d.as_ref())))
            .collect();

        let root = if leaves.is_empty() {
            None
        } else {
            Some(Self::build(leaves.clone()))
        };

        MerkleTree { root, leaves }
    }

    fn build(mut nodes: Vec<String>) -> String {
        while nodes.len() > 1 {
            if !nodes.len().is_multiple_of(2) {
                // Duplicate last node for odd levels
                nodes.push(nodes.last().cloned().unwrap());
            }
            nodes = nodes
                .chunks(2)
                .map(|pair| {
                    let mut h = Sha256::new();
                    h.update(pair[0].as_bytes());
                    h.update(pair[1].as_bytes());
                    encode(h.finalize())
                })
                .collect();
        }
        nodes.remove(0)
    }

    /// Return the root hash, or `None` for an empty tree.
    pub fn root(&self) -> Option<&str> {
        self.root.as_deref()
    }

    /// Return the leaf hashes (one per original data element).
    pub fn leaves(&self) -> &[String] {
        &self.leaves
    }

    /// Verify that `data[index]` still produces the expected leaf hash,
    /// i.e. the data has not been tampered with.
    pub fn verify_leaf(&self, index: usize, data: impl AsRef<[u8]>) -> bool {
        let expected = encode(Sha256::digest(data.as_ref()));
        self.leaves
            .get(index)
            .map(|l| l == &expected)
            .unwrap_or(false)
    }

    /// Check if two trees have identical root hashes (same dataset or same state).
    pub fn matches(&self, other: &MerkleTree) -> bool {
        self.root == other.root
    }
}
