// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(
    // missing_docs,
    rust_2024_compatibility,
    // trivial_casts,
    // unused_lifetimes,
    // unused_qualifications
)]

use sops_gitops_github_action::{get_pubkey_fingerprint, read_public_key, set_message};

use std::sync::LazyLock;

const MOCK_KEY_FOOTPRINT: &str = "9C243A3FDC4EF1474372915F9C1B6F1F746AF12C";

#[allow(dead_code)]
static GPG_MOCK_PUBLIC_KEY: LazyLock<String> =
    LazyLock::new(|| read_public_key("tests/mock-public.key.asc"));

#[test]
fn test_read_public_key() {
    let key = read_public_key("tests/mock-public.key.asc");
    assert!(key.contains("-----BEGIN PGP PUBLIC KEY BLOCK-----"));
}

#[test]
fn test_get_key_fingerprint() {
    let key = read_public_key("tests/mock-public.key.asc");
    let result = get_pubkey_fingerprint(&key).expect("Failed to get fingerprint");
    assert_eq!(result, MOCK_KEY_FOOTPRINT);
}

#[test]
fn test_set_message() {
    let result = set_message();
    assert!(result.is_ok());
}
