use derive_bip_39_seed::*;

fn main() -> Result<(), DerivationError> {
    print!("{:?}", mnemonic_to_bytes("tube tired cage message bar language sock trap speak lonely uncover brief")?);
    Ok(())
}