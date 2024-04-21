use backend::utils::fingerprint::{generate_fingerprint, verify_fingerprint_hash};

async fn fingerprint_test() {
    let fingerprint = generate_fingerprint().unwrap();

    let verify = verify_fingerprint_hash(&fingerprint.0, &fingerprint.1).unwrap();

    // Assert
    assert!(verify);
}
