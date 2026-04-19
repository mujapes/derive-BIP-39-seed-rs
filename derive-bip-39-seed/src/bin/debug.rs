use derive_bip_39_seed::mnemonic_to_bytes;

fn main() {
    print!("{:?}", mnemonic_to_bytes("tube tired cage message bar language sock trap speak lonely uncover brief").unwrap());
}