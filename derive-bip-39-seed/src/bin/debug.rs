use derive_bip_39_seed::*;

fn main() -> Result<(), DerivationError> {
    print!("{:?}", mnemonic_to_bytes("labor great follow frame drip auction vanish nut load enemy used coconut")?);
    print!("{:?}", mnemonic_to_bytes("tube tired cage message bar language sock trap speak lonely uncover brief")?);
    for b in mnemonic_to_bytes("tube tired cage message bar language sock trap speak lonely uncover brief")?.0 {
        println!("{:08b}", b);
    }
    // {:08b}
    // expected: 234, 28, 
    Ok(())
}