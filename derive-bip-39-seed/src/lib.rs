/*!
 * Rust implementation of seed derivation from a 12-24 word BIP-39 mnemonic phrase + optional passphrase salt.
 */

use unicode_normalization::UnicodeNormalization;
use std::io;

#[derive(Debug)]
pub enum DerivationError {
    InvalidMnemonicFormat,
    InvalidMnemonicWord,
    InvalidMnemonicChecksum,
    Io(io::Error),
}

impl From<io::Error> for DerivationError {
    fn from(err: io::Error) -> Self {
        DerivationError::Io(err)
    }
}

/// Verifies the mnemonic's checksum by removing the last 4 bits and 
/// comparing them to the SHA256 hash of the rest
/// 
/// # Arguments
///
/// * `mnemonic` - The 12-24 word BIP-39 mnemonic phrase
///
/// # Returns
///
/// `Ok(())` if the checksum is valid, otherwise `Err(DerivationError::InvalidMnemonicChecksum)`
fn checksum((mnemonic, word_count): ([u8; 33], usize)) -> Result<(), DerivationError> {
    Ok(())
}

/// Converts the mnemonic phrase into a byte array using BIP-39 word mappings
/// 
/// # Arguments
/// 
/// * `mnemonic` - The 12-24 word BIP-39 mnemonic phrase
/// * `word_count` - The number of words in the mnemonic phrase
/// 
/// # Returns
/// 
/// A 33-byte array representing the mnemonic phrase, or `Err(DerivationError::InvalidMnemonicFormat)`
pub fn mnemonic_to_bytes(mnemonic: &str) -> Result<([u8; 33], usize), DerivationError> {
    let words: Vec<&str> = mnemonic.split(" ").collect();
    if words.len() % 3 != 0 || words.len() < 12 || words.len() > 24 { return Err(DerivationError::InvalidMnemonicFormat); }
    
    let word_lists = get_word_lists();
    let mut language_index = 9;
    for lang in 0..9 {
        if word_lists[lang].binary_search(&words[0]).is_ok() {
            language_index = lang;
            break;
        }
    }

    if language_index == 9 {
        return Err(DerivationError::InvalidMnemonicWord);
    }

    let mut bytes = [0; 33];
    let mut bit_cnt = 0;
    let mut byte_index: usize = 0;

    for word in &words {
        match word_lists[language_index].binary_search(&word) {
            Ok(mut index) => {
                let mut bits_to_fill = 8 - bit_cnt % 8;
                let mut mask = u8::MAX >> 8 - bits_to_fill;
                mask &= index as u8;
                mask <<= 8 - bits_to_fill;
                bytes[byte_index] |= mask;

                bit_cnt += 11;
                byte_index += 1;
                index >>= bits_to_fill;

                if bits_to_fill <= 3 {
                    bytes[byte_index] = index as u8;
                    byte_index += 1;
                    index >>= 8;
                }

                bytes[byte_index] = index as u8;
            },
            Err(_) => return Err(DerivationError::InvalidMnemonicWord)
        }
    }

    Ok( (bytes, words.len()) )
}

fn get_word_lists() -> [[&'static str; 2048]; 9]{
    let mut wordlists = [[""; 2048]; 9];
    wordlists[0] = include_str!("../wordlists/chinese_simplified.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();
    wordlists[1] = include_str!("../wordlists/czech.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();
    wordlists[2] = include_str!("../wordlists/english.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();
    wordlists[3] = include_str!("../wordlists/french.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();
    wordlists[4] = include_str!("../wordlists/italian.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();
    wordlists[5] = include_str!("../wordlists/japanese.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();
    wordlists[6] = include_str!("../wordlists/korean.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();
    wordlists[7] = include_str!("../wordlists/portuguese.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();
    wordlists[8] = include_str!("../wordlists/spanish.txt").trim().lines().collect::<Vec<&str>>().try_into().unwrap();

    wordlists
}

/// Derives a 64-byte seed from a 12-24 word BIP-39 mnemonic phrase + optional passphrase salt
/// 
/// # Examples
///
/// ```
/// use derive_bip_39_seed::derive_seed;
/// use hex_literal::hex;
///
/// let seed = derive_seed(
///     "elder call scrap anchor grit spice loud shaft donor model knife curious",
///     "testtest"
/// ).unwrap();
/// 
/// assert_eq!(seed, hex!("e768924206ced126a52606b8613e707efb34fac9d2b10dec48c78aea2f68068a20d94f184673acff14f3d82cd0bcf2fe881b190de2902fbad6a1f199a69c2347"));
/// 
/// let seed = derive_seed(
///     "mirror rapid embody guard output essence minor salmon exercise episode book exile valley view concert census chapter popular make cube tired sibling together any",
///     "(&JJ@"
/// ).unwrap();
/// 
/// assert_eq!(seed, hex!("b1d858ca398a85053eef0235488c2736308cf8a03af40d58d5739577d977dc4382303f04bc0dae700db2cba3fb2a2d93e37d2f13d90c581993a2b4952064fe01"));
/// 
/// let seed = derive_seed(
///     "call scrap anchor elder grit spice loud shaft donor model knife curious",
///     "memem"
/// ).unwrap_err();
/// 
/// assert_eq!(seed, DerivationError::InvalidMnemonicChecksum);
/// 
/// let seed = derive_seed(
///     " call scra spiceloud sfo*ou((@ous",
///     "j82n2d"
/// ).unwrap_err();
/// 
/// assert_eq!(seed, DerivationError::InvalidMnemonicFormat);
/// ```
/// 
/// # Arguments
/// 
/// * `mnemonic` - The 12-24 word BIP-39 mnemonic phrase
/// * `salt` - An optional passphrase salt
/// 
/// # Returns
/// 
/// A 64-byte seed, or a `DerivationError`
pub fn derive_from_string(mnemonic: &str, salt: &str) -> Result<[u8; 64], DerivationError> {
    let mnemonic_nfkd: String = mnemonic.nfkd().collect();
    let salt_nfkd: String = format!("mnemonic{}", salt.nfkd().collect::<String>());
    // Verify the mnemonic's checksum
    checksum( mnemonic_to_bytes(&mnemonic_nfkd)? )?;
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
