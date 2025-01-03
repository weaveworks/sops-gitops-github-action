// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(
    // missing_docs,
    rust_2024_compatibility,
    // trivial_casts,
    // unused_lifetimes,
    // unused_qualifications
)]

use base64::Engine;
use pgp::ser::Serialize;
use pgp::{Deserializable, Message, SignedPublicKey};
use sops_gitops_github_action::{
    create_default_sops_config_file, create_secret_file, find_secret_files, get_key_fingerprint,
    get_pubkey_fingerprint, gpg_mock_private_key, import_gpg_key, public_keys_provided,
    read_public_key, set_message, sops_config_file_exists, update_secret_file, update_sops_config,
};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::sync::LazyLock;
use tempfile::tempdir;

const MOCK_KEY_FOOTPRINT: &str = "9C243A3FDC4EF1474372915F9C1B6F1F746AF12C";
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
