use crate::{hash, hash_pair};

pub struct MerkleTree {
    hashes: Vec<Vec<u8>>,
    count: usize
}

impl MerkleTree {
    pub fn new(elements: &[String]) -> MerkleTree{
        let mut tree = MerkleTree{hashes: vec![], count:  elements.len()};
        let leaves = tree.hash_elements(elements);
        tree.hashes = tree.build_hashes(leaves);
        tree
    }

    pub fn root_hash(&self) -> &[u8] {
        &self.hashes[0]
    }

    pub fn add(&mut self, elements: &[String]) {
        let new_leaves = self.hash_elements(&elements);
        let old_leaves = self.hashes[{self.count-1}..].to_vec();
        let leaves = [old_leaves, new_leaves].concat();
        let count =  leaves.len();
        self.hashes = self.build_hashes(leaves);
        self.count = count;
    }

    fn hash_elements(&self, elements: &[String]) -> Vec<Vec<u8>> {
        elements.iter().map(|e| hash(e.to_string())).collect()
    }

    fn build_hashes(&self, hashes: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        if hashes.len() == 1 {
            return hashes;
        }

        let h: Vec<Vec<u8>>= hashes.chunks(2).clone().into_iter().map(|e| hash_pair(e[0].clone(), e[1].clone())).collect();
        [self.build_hashes(h), hashes].concat()
    }
}


#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use crate::merkle_tree::*;

    #[test]
    fn root_hash_of_hola_moikka_is_correct() {
        let tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string()]);

        assert_eq!(tree.root_hash(), hex!("d703ed960de71d89e617a637f87813b9da95461f30d5d5030329b979ff931032"));
    }

    #[test]
    fn when_adding_two_more_elements_to_the_tree_the_root_hash_is_correct() {
        let mut tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string()]);
        tree.add( &["heippa".to_string(), "ahoj".to_string()]);
        assert_eq!(tree.root_hash(), hex!("8321751cd2de3135bcc3ee9ad978061b284d1ec23f83279192ebcc3666c9e5cc"));
    }
}