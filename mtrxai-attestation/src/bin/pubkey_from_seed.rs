use mtrxai_attestation::signing_key_from_seed_hex;

fn main() {
    let seed_hex = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("MTRXAI_ATTESTATION_SECRET").ok())
        .expect("usage: pubkey-from-seed <seed-hex>");
    let signing_key = signing_key_from_seed_hex(&seed_hex).expect("valid seed");
    println!("{}", hex::encode(signing_key.verifying_key().to_bytes()));
}
