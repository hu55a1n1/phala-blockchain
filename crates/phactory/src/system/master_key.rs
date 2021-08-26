use crate::pal::Sealing;
use phala_crypto::sr25519::{Persistence, Signing, SECRET_KEY_LENGTH, SIGNATURE_BYTES};
use sp_core::sr25519;
use std::path::PathBuf;

/// Master key filepath
pub const MASTER_KEY_FILE: &str = "master_key.seal";

fn master_key_file_path(sealing_path: String) -> PathBuf {
    PathBuf::from(&sealing_path).join(MASTER_KEY_FILE)
}

/// Seal master key seed with signature to ensure integrity
pub fn seal(
    sealing_path: String,
    master_key: &sr25519::Pair,
    identity_key: &sr25519::Pair,
    sys: &impl Sealing,
) {
    let secret = master_key.dump_secret_key();
    let sig = identity_key.sign_data(&secret);

    // TODO(shelven): use serialization rather than manual concat.
    let mut buf = Vec::new();
    buf.extend_from_slice(&secret);
    buf.extend_from_slice(sig.as_ref());

    let filepath = master_key_file_path(sealing_path);
    info!("Seal master key to {}", filepath.as_path().display());

    sys.seal_data(filepath, &buf)
        .expect("Seal master key failed");
}

/// Unseal local master key seed and verify signature
///
/// This function could panic a lot.
pub fn try_unseal(
    sealing_path: String,
    identity_key: &sr25519::Pair,
    sys: &impl Sealing,
) -> Option<sr25519::Pair> {
    let filepath = master_key_file_path(sealing_path);
    info!("Unseal master key from {}", filepath.as_path().display());

    let mut secret = [0_u8; SECRET_KEY_LENGTH];
    let mut sig = [0_u8; SIGNATURE_BYTES];

    let data = sys.unseal_data(filepath).expect("Unseal master key failed");
    let data = match data {
        Some(data) => data,
        None => {
            info!("No sealed master key");
            return None;
        }
    };

    todo!("Load secret");
}
