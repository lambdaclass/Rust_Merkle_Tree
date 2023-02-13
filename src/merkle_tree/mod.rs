pub struct MerkleTree {
    hashes: Vec<String>,
    count: usize
}

impl MerkleTree {
    pub fn new(elements: Vec<String>) -> MerkleTree{
        MerkleTree{hashes: elements.clone(), count: elements.len()}
    }
}


#[cfg(test)]
mod tests {
    use crate::merkle_tree::*;

    #[test]
    fn test() {
        assert!(true);
    }
}