use hex_literal::hex;
use sha3::{Digest, Sha3_256};

pub fn hash(element: String) -> Vec<u8> {
    let mut hasher = Sha3_256::new();

    hasher.update(element);
    hasher.finalize().to_vec()
}

pub fn hash_pair(a: Vec<u8>, b: Vec<u8>) -> Vec<u8>{
    let mut hasher = Sha3_256::new();

    hasher.update([a, b].concat());
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use crate::hasher::*;

    #[test]
    fn hash_of_hola_is_correct() {
        let hola = "hola".to_string();
        assert_eq!(hash(hola), hex!("8af13d9244618eee876d0431f3449aa4ff95274ca3e7e5c6541979499f5b85de"));
    }

    #[test]
    fn hash_of_moikka_is_correct() {
        let hola = "moikka".to_string();
        assert_eq!(hash(hola), hex!("97af0c1bd5cdfd6250fe6cd7a74c11f00e21b7c74014dd9ade9102118d72e8e4"));
    }

    #[test]
    fn hash_of_hola_moikka_is_correct() {
        let hola = hex!("8af13d9244618eee876d0431f3449aa4ff95274ca3e7e5c6541979499f5b85de").to_vec();
        let moikka = hex!("97af0c1bd5cdfd6250fe6cd7a74c11f00e21b7c74014dd9ade9102118d72e8e4").to_vec();

        assert_eq!(hash_pair(hola, moikka), hex!("d703ed960de71d89e617a637f87813b9da95461f30d5d5030329b979ff931032"));
    }
}
