use crate::{hash, hash_pair};

pub struct MerkleTree {
    hashes: Vec<Vec<u8>>,
    count: usize
}

impl MerkleTree {
    pub fn new(elements: Vec<String>) -> MerkleTree{
        let hashes = elements.iter().map(|e| hash(e.to_string())).collect();
        MerkleTree{hashes, count: elements.len()}
    }

    pub fn root_hash(tree: MerkleTree) -> Vec<u8> {
        Self::get_root_hash(tree.hashes)
    }

    fn get_root_hash(hashes: Vec<Vec<u8>>) -> Vec<u8> {
        if hashes.len() == 1 {
            return hashes[0].clone();
        }

        let h = hashes.chunks(2).map(|e| hash_pair(e[0].clone(), e[1].clone())).collect();
        Self::get_root_hash(h)
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
}