use sha3::{Digest, Sha3_256};

pub fn hash(element: String) -> Vec<u8> {
    let mut hasher = Sha3_256::new();

    hasher.update(element);
    hasher.finalize().to_vec()
}
