use sha2::{Digest, Sha256};

/// A hash is represented as a 32-byte array (SHA-256).
pub type Hash = [u8; 32];

pub struct MerkleTree {
    nodes: Vec<Hash>,
    leaf_count: usize,
    capacity: usize,
}

impl MerkleTree {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity.is_power_of_two(), "Capacity must be a power of two (2^n)");
        let total_nodes = 2 * capacity - 1;
        Self {
            nodes: vec![[0u8; 32]; total_nodes],
            leaf_count: 0,
            capacity,
        }
    }

    pub fn hash_leaf(data: &[u8]) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update([0u8]);
        hasher.update(data);
        hasher.finalize().into()
    }

    pub fn hash_internal(left: &Hash, right: &Hash) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update([1u8]); // Internal node domain separator
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().into()
    }
    pub fn add_leaf(&mut self, data: &[u8]) {
        assert!(self.leaf_count < self.capacity, "Merkle Tree capacity reached");
        
        let leaf_idx = self.capacity - 1 + self.leaf_count;
        self.nodes[leaf_idx] = Self::hash_leaf(data);
        
        // Recompute hashes along the path to the root.
        self.update_path(self.leaf_count);
        
        self.leaf_count += 1;
    }

    pub fn update_leaf(&mut self, index: usize, data: &[u8]) {
        assert!(index < self.leaf_count, "Leaf index out of bounds");
        
        let leaf_idx = self.capacity - 1 + index;
        self.nodes[leaf_idx] = Self::hash_leaf(data);
        
        self.update_path(index);
    }

    fn update_path(&mut self, leaf_index: usize) {
        let mut current_idx = self.capacity - 1 + leaf_index;
        
        while current_idx > 0 {
            // Move to parent
            let parent_idx = (current_idx - 1) / 2;
            
            let (left_idx, right_idx) = if current_idx % 2 == 1 {
                // If odd, current is the left child
                (current_idx, current_idx + 1)
            } else {
                // If even, current is the right child
                (current_idx - 1, current_idx)
            };
            
            self.nodes[parent_idx] = Self::hash_internal(&self.nodes[left_idx], &self.nodes[right_idx]);
            current_idx = parent_idx;
        }
    }

    /// Returns the root hash of the Merkle Tree.
    pub fn root(&self) -> Hash {
        self.nodes[0]
    }

    /// Generates a Merkle Proof for the leaf at the given index.
    /// Returns a vector of tuples: (Sibling Hash, is_left_child_of_parent).
    /// The proof consists of sibling hashes along the path to the root.
    pub fn get_proof(&self, index: usize) -> Vec<(Hash, bool)> {
        assert!(index < self.leaf_count, "Leaf index out of bounds");
        
        let mut proof = Vec::new();
        let mut current_idx = self.capacity - 1 + index;
        
        while current_idx > 0 {
            let is_left = current_idx % 2 == 1;
            let sibling_idx = if is_left {
                current_idx + 1
            } else {
                current_idx - 1
            };
            
            // Store sibling hash and whether the CURRENT node (not sibling) is a left child.
            proof.push((self.nodes[sibling_idx], is_left));
            current_idx = (current_idx - 1) / 2;
        }
        
        proof
    }

    /// Verifies a Merkle Proof against a root hash and leaf data.
    pub fn verify_proof(root: &Hash, leaf_hash: &Hash, proof: &[(Hash, bool)]) -> bool {
        let mut current_hash = *leaf_hash;
        
        for (sibling_hash, was_left) in proof {
            if *was_left {
                // Current node was left, sibling is right
                current_hash = Self::hash_internal(&current_hash, sibling_hash);
            } else {
                // Current node was right, sibling is left
                current_hash = Self::hash_internal(sibling_hash, &current_hash);
            }
        }
        
        &current_hash == root
    }

    /// Utility to get current leaf count
    pub fn leaf_count(&self) -> usize {
        self.leaf_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_construction() {
        let mut tree = MerkleTree::new(4);
        tree.add_leaf(b"leaf 0");
        tree.add_leaf(b"leaf 1");
        tree.add_leaf(b"leaf 2");
        tree.add_leaf(b"leaf 3");
        
        assert_eq!(tree.leaf_count(), 4);
        // Correctness of root is implicitly tested by proof verification
    }

    #[test]
    fn test_proof_verification() {
        let mut tree = MerkleTree::new(4);
        let data: Vec<&[u8]> = vec![b"apple", b"banana", b"cherry", b"date"];
        for d in &data {
            tree.add_leaf(*d);
        }
        
        let root = tree.root();
        
        for (i, d) in data.iter().enumerate() {
            let leaf_hash = MerkleTree::hash_leaf(*d);
            let proof = tree.get_proof(i);
            assert!(MerkleTree::verify_proof(&root, &leaf_hash, &proof), "Proof failed for index {}", i);
        }
    }

    #[test]
    fn test_update_leaf() {
        let mut tree = MerkleTree::new(2);
        tree.add_leaf(b"A");
        tree.add_leaf(b"B");
        
        let old_root = tree.root();
        
        tree.update_leaf(0, b"C");
        let new_root = tree.root();
        
        assert_ne!(old_root, new_root);
        
        let leaf_hash = MerkleTree::hash_leaf(b"C");
        let proof = tree.get_proof(0);
        assert!(MerkleTree::verify_proof(&new_root, &leaf_hash, &proof));
    }
}
