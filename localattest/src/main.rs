use crate::attestation::Attestation;

mod attestation;

fn main() {
    // Generate report on the fly
    let ti = Attestation::get_own_targetinfo();
    println!("Target info: {:x?}", ti);
    let mut report = Attestation::locally_attest(&ti, &[0u8; 64])
        .expect("Could not attest for own target info.");
    println!("Report: {:x?}", report);
    
    // Report should be invalid when uncommented:
    //report[0] ^= 1u8;

    println!("Valid: {}", Attestation::verify_local_attest(&report));
}
