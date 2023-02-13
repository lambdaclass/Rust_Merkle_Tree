use crate::{hash, hash_pair};

pub struct MerkleTree {
    hashes: Vec<Vec<u8>>,
    count: usize
}

impl MerkleTree {
    pub fn new(elements: Vec<String>) -> MerkleTree{
        let leaves = elements.iter().map(|e| hash(e.to_string())).collect();
        let hashes = Self::build_hashes(leaves);
        MerkleTree{hashes, count: elements.len()}
    }

    pub fn root_hash(tree: MerkleTree) -> Vec<u8> {
        tree.hashes[0].clone()
    }

    fn build_hashes(hashes: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        if hashes.len() == 1 {
            return hashes;
        }

        let h: Vec<Vec<u8>>= hashes.chunks(2).map(|e| hash_pair(e[0].clone(), e[1].clone())).collect();
        [Self::build_hashes(h.clone()), h].concat()
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