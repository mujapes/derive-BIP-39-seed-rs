use derive_bip_39_seed::*;
use rand::SeedableRng; // Requirement for .from_seed() or .seed_from_u64()
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

fn main() -> Result<(), DerivationError> {
    println!("{:?}", mnemonic_to_bytes("labor great follow frame drip auction vanish nut load enemy used coconut"));
    println!("{:?}", mnemonic_to_bytes("tube tired cage message bar language sock trap speak lonely uncover brief"));
    println!("labor great follow frame drip auction vanish nut load enemy used coconut");
    println!("{:?}", derive_from_string("labor great follow frame drip auction vanish nut load enemy used coconut", ""));
    println!("tube tired cage message bar language sock trap speak lonely uncover brief");
    println!("{:?}", derive_from_string("tube tired cage message bar language sock trap speak lonely uncover brief", ""));
    println!("labor great follow frame drip auction vanish nut load enemy coconut used");
    println!("{:?}", derive_from_string("labor great follow frame drip auction vanish nut load enemy coconut used", ""));
    println!("tube tired cage message bar language sock trap speak lonely brief uncover");
    println!("{:?}", derive_from_string("tube tired cage message bar language sock trap speak lonely brief uncover", ""));
    println!("ill window heart advance tunnel hurdle canal like seminar basic local south");
    println!("{:?}", derive_from_string("ill window heart advance tunnel hurdle canal like seminar basic local south", ""));
    println!("point genius faculty shrimp stick build audit keep beyond visit mesh math");
    println!("{:?}", derive_from_string("point genius faculty shrimp stick build audit keep beyond visit mesh math", ""));
    println!("museum arctic pool butter lyrics gorilla drift trap escape hospital inject emerge");
    println!("{:?}", derive_from_string("museum arctic pool butter lyrics gorilla drift trap escape hospital inject emerge", ""));
    println!("enemy load follow coconut drip frame great labor vanish auction nut used");
    println!("{:?}", derive_from_string("enemy load follow coconut drip frame great labor vanish auction nut used", ""));
    println!("great auction drip enemy load follow coconut vanish used frame labor nut");
    println!("{:?}", derive_from_string("great auction drip enemy load follow coconut vanish used frame labor nut", ""));
    println!("enemy follow frame vanish drip great load nut used coconut labor auction");
    println!("{:?}", derive_from_string("enemy follow frame vanish drip great load nut used coconut labor auction", ""));
    // {:08b}
    // expected: 234, 28, 
    let mut words = "labor great follow frame drip auction vanish nut load enemy used coconut".split(" ").collect::<Vec<&str>>();
    let mut rng = StdRng::seed_from_u64(42);

    /*for i in 0..50 {
        words.shuffle(&mut rng);
        let phrase = words.join(" ");
        println!("{}", &phrase);
        println!("{:?}", derive_from_string(&phrase, ""));
    }*/

    Ok(())
}

// False Positives
// enemy load follow coconut drip frame great labor vanish auction nut used
// 01001001111 10000010111 01011010101 00101100111 01000011001 01011100100
// 01100110001 01111100010 11110001010 00001110111 10010111101 11101111110
// great auction drip enemy load follow coconut vanish used frame labor nut
// 01100111010 00001111000 01000100000 01001010110 10000100000 01011011101
// 00101101110 11110010111 11110001011 01011101100 01111101011 10011000110

// False Negatives
// enemy follow frame vanish drip great load nut used coconut labor auction
// ill window heart advance tunnel hurdle canal like seminar basic local south
// point genius faculty shrimp stick build audit keep beyond visit mesh math
// museum arctic pool butter lyrics gorilla drift trap escape hospital inject emerge