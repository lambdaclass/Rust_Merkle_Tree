use crate::{hash, hash_pair};

pub struct MerkleTree {
    hashes: Vec<Vec<u8>>,
    count: usize
}

impl MerkleTree {
    pub fn new(elements: Vec<String>) -> MerkleTree{
        let leaves = Self::hash_elements(elements.clone());
        let hashes = Self::build_hashes(leaves);
        MerkleTree{hashes, count: elements.len()}
    }

    pub fn root_hash(tree: MerkleTree) -> Vec<u8> {
        tree.hashes[0].clone()
    }

    pub fn add(tree: MerkleTree, elements: Vec<String>) -> MerkleTree {
        let new_leaves = Self::hash_elements(elements);
        let old_leaves = tree.hashes[{tree.count-1}..].to_vec();
        let leaves = [old_leaves, new_leaves].concat();
        let hashes = Self::build_hashes(leaves.clone());
        MerkleTree{hashes, count: leaves.len()}
    }

    fn hash_elements(elements: Vec<String>) -> Vec<Vec<u8>> {
        elements.iter().map(|e| hash(e.to_string())).collect()
    }

    fn build_hashes(hashes: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        if hashes.len() == 1 {
            return hashes;
        }

        let h: Vec<Vec<u8>>= hashes.chunks(2).map(|e| hash_pair(e[0].clone(), e[1].clone())).collect();
        [Self::build_hashes(h), hashes].concat()
    }
}


#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use crate::merkle_tree::*;

    #[test]
    fn root_hash_of_hola_moikka_is_correct() {
        let tree = MerkleTree::new(vec!["hola".to_string(), "moikka".to_string()]);

        assert_eq!(MerkleTree::root_hash(tree), hex!("d703ed960de71d89e617a637f87813b9da95461f30d5d5030329b979ff931032"));
    }

    #[test]
    fn when_adding_two_more_elements_to_the_tree_the_root_hash_is_correct() {
        let tree = MerkleTree::new(vec!["hola".to_string(), "moikka".to_string()]);
        let new_tree = MerkleTree::add(tree, vec!["heippa".to_string(), "ahoj".to_string()]);
        assert_eq!(MerkleTree::root_hash(new_tree), hex!("8321751cd2de3135bcc3ee9ad978061b284d1ec23f83279192ebcc3666c9e5cc"));
    }
}