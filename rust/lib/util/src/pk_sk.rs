use ed25519_dalek_blake3::Keypair;
use rand::rngs::OsRng;

pub fn pk_sk() -> ([u8; 32], [u8; 32]) {
  let pair = Keypair::generate(&mut OsRng {});
  let pk = pair.public.as_bytes();
  let sk = pair.secret.as_bytes();
  (*pk, *sk)
}
