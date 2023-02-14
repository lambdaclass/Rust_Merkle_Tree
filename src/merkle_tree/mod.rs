use std::ops::Rem;

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

    fn proof(&self, mut index: usize) -> Vec<Vec<u8>> {
        let mut proof = vec![];
        let mut i = self.count-1;

        while i != 0 {
            let h: Vec<u8> = match index.rem(2) {
                0 => self.hashes[index+1+i].clone(),
                _ => self.hashes[index+i-1].clone()
            };
            proof.append(&mut vec![h]);
            index = index/2;
            i = {i+1}/2 - 1;
        }
        proof
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

    #[test]
    fn proof_for_the_first_element_of_four() {
        let tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string(), "heippa".to_string(), "ahoj".to_string()]);
        let moikka = hash("moikka".to_string());
        let heippa = hash("heippa".to_string());
        let ahoj = hash("ahoj".to_string());
        let heippa_ahoj = hash_pair(heippa, ahoj);
        let expected_proof = vec![moikka, heippa_ahoj];
        assert_eq!(tree.proof(0), expected_proof);
    }

    #[test]
    fn proof_for_the_second_element_of_four() {
        let tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string(), "heippa".to_string(), "ahoj".to_string()]);
        let hola = hash("hola".to_string());
        let heippa = hash("heippa".to_string());
        let ahoj = hash("ahoj".to_string());
        let heippa_ahoj = hash_pair(heippa, ahoj);
        let expected_proof = vec![hola, heippa_ahoj];
        assert_eq!(tree.proof(1), expected_proof);
    }

    #[test]
    fn proof_for_the_third_element_of_four() {
        let tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string(), "heippa".to_string(), "ahoj".to_string()]);
        let hola = hash("hola".to_string());
        let moikka = hash("moikka".to_string());
        let ahoj = hash("ahoj".to_string());
        let hola_moikka = hash_pair(hola, moikka);
        let expected_proof = vec![ahoj, hola_moikka];
        assert_eq!(tree.proof(2), expected_proof);
    }

    #[test]
    fn proof_for_the_fourth_element_of_four() {
        let tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string(), "heippa".to_string(), "ahoj".to_string()]);
        let hola = hash("hola".to_string());
        let moikka = hash("moikka".to_string());
        let heippa = hash("heippa".to_string());
        let hola_moikka = hash_pair(hola, moikka);
        let expected_proof = vec![heippa, hola_moikka];
        assert_eq!(tree.proof(3), expected_proof);
    }

    #[test]
    fn proof_for_the_fourth_element_of_eight() {
        let elements = [
            "hola".to_string(), 
            "moikka".to_string(), 
            "heippa".to_string(), 
            "ahoj".to_string(),
            "privet".to_string(), 
            "bonjour".to_string(), 
            "konichiwa".to_string(),
            "rytsas".to_string()
        ];
        let tree = MerkleTree::new(&elements);
        let hola = hash("hola".to_string());
        let moikka = hash("moikka".to_string());
        let heippa = hash("heippa".to_string());
        let privet = hash("privet".to_string());
        let bonjour = hash("bonjour".to_string());
        let konichiwa = hash("konichiwa".to_string());
        let rytsas = hash("rytsas".to_string());
        let hola_moikka = hash_pair(hola, moikka);
        let privet_bonjour = hash_pair(privet, bonjour);
        let konichiwa_rytsas = hash_pair(konichiwa, rytsas);
        let privet_bonjour_konichiwa_rytsas = hash_pair(privet_bonjour, konichiwa_rytsas);
        let expected_proof = vec![heippa, hola_moikka, privet_bonjour_konichiwa_rytsas];
        assert_eq!(tree.proof(3), expected_proof);
    }
}