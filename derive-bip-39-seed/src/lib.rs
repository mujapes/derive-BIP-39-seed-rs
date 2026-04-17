use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub enum DerivationError {
    InvalidMnemonic,
    InvalidChecksum,
    InvalidSalt,
}
// USE &[U8] INSTEAD OF VECS where possible

fn checksum(mnemonic: &str) -> Result<bool, DerivationError> {
    mnemonic_to_bytes(mnemonic);
    Ok(false)
}

fn mnemonic_to_bytes(mnemonic: &str) -> [u8; 33] {
    [0; 33]
}

fn validate_mnemonic(mnemonic: &str) -> Result<bool, DerivationError> {
    // bip 39 supports 12 languages
    Ok(false)
}


pub fn derive_from_string(mnemonic: &str, salt: &str) -> Result<[u8; 64], DerivationError> {
    let mnemonic_nfkd: String = mnemonic.nfkd().collect();
    let salt_nfkd: String = salt.nfkd().collect();

    // pbkdf2 crate call
    Ok([0; 64])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = "test";
        assert_eq!(result, "test");
    }
}
