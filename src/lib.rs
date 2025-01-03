use anyhow::{Error, anyhow, Context as AnyhowContext};
use base64::Engine;
use clap::Parser;
use glob::glob;
use serde_yaml::Value;
use std::fs;
use std::fs::File;
use gpgme::{Context as GpgmeContext, Protocol};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use pgp::{SignedPublicKey, Message, Deserializable};
use pgp::ser::Serialize;
use pgp::types::PublicKeyTrait;
use sequoia_openpgp as openpgp;
use openpgp::{parse::Parse, Cert};
#[derive(Parser, Debug)]
struct Args {
    /// Base64-encoded private GPG key
    #[clap(long)]
    private_key: String,
    /// Comma-separated list of base64-encoded public GPG keys
    #[clap(long)]
    public_keys: String,
}

// let pub_key_file = "tests/mock-public.key.asc";

// get_private_key is a helper function that returns the private key from GitHub secrets
// or the environment variable GPG_MOCK_PRIVATE_KEY
pub fn gpg_mock_private_key() -> Result<String, Error> {
    std::env::var("GPG_MOCK_PRIVATE_KEY")
        .context("GPG_MOCK_PRIVATE_KEY environment variable not set")
}

pub fn read_public_key(pub_key_file: &str) -> String {
    let key_string = fs::read_to_string(pub_key_file).unwrap();
    return key_string
}

pub fn get_pubkey_fingerprint(armored_pubkey: &str) -> Result<String, Error>  {
    let cert = Cert::from_reader(armored_pubkey.as_bytes())
        .map_err(|err| anyhow!("Failed to parse armored public key: {err}"))?;

    let primary_key = cert
        .keys()
        .next()
        .ok_or_else(|| anyhow!("No keys found in the given public key data"))?
        .key();

    let fingerprint_hex = primary_key.fingerprint().to_hex();

    Ok(fingerprint_hex)
}

pub fn import_gpg_key(key_data: &str) -> anyhow::Result<(), anyhow::Error> {
    let key = SignedPublicKey::from_bytes(key_data.as_bytes())
        .map_err(|e| anyhow!("Failed to parse PGP key: {e}"))?;

    key.fingerprint().as_bytes().iter().for_each(|byte| print!("{:02X}", byte));

    Ok(())
}

pub fn update_sops_config(public_keys: &str) -> anyhow::Result<(), anyhow::Error> {
    let sops_config_path = "./.sops.yaml";
    let mut config: Value = if Path::new(sops_config_path).exists() {
        let content =
            fs::read_to_string(sops_config_path).context("Failed to read existing .sops.yaml")?;
        serde_yaml::from_str(&content).context("Failed to parse .sops.yaml")?
    } else {
        serde_yaml::from_str("creation_rules: []").unwrap()
    };

    // Update the creation rules with public keys
    let creation_rules = config
        .get_mut("creation_rules")
        .and_then(Value::as_sequence_mut)
        .ok_or_else(|| anyhow!("Invalid .sops.yaml structure"))?;

    for public_key in public_keys.split(',') {
        let fingerprint = get_key_fingerprint(public_key)?;
        let rule = serde_yaml::to_value(&serde_yaml::Mapping::from_iter([(
            Value::String("pgp".to_string()),
            Value::Sequence(vec![Value::String(fingerprint)]),
        )]))?;
        creation_rules.push(rule);
    }

    // Write back the updated configuration
    let updated_config =
        serde_yaml::to_string(&config).context("Failed to serialize updated .sops.yaml")?;
    fs::write(sops_config_path, updated_config).context("Failed to write updated .sops.yaml")?;
    Ok(())
}


pub fn get_key_fingerprint(encoded_key: &str) -> anyhow::Result<String> {
    let decoded_key = base64::engine::general_purpose::STANDARD
        .decode(encoded_key)
        .context("Failed to decode base64 key")?;

    let mut child = Command::new("gpg")
        .args([
            "--with-colons",
            "--import-options",
            "show-only",
            "--import",
            "--fingerprint",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to spawn gpg process")?;

    // Write the decoded key to GPG’s stdin
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(&decoded_key)
        .context("Failed to write key to gpg stdin")?;

    let output = child
        .wait_with_output()
        .context("Failed to wait for gpg process")?;

    let output_str =
        String::from_utf8(output.stdout).context("Failed to parse gpg output")?;

    let fingerprint = output_str
        .lines()
        .find(|line| line.starts_with("fpr"))
        .and_then(|line| line.split(':').nth(9))
        .map(String::from)
        .context("Failed to extract fingerprint from gpg output")?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&fingerprint);

    Ok(encoded)
}


#[allow(unused)]
pub fn setup_workspace() -> anyhow::Result<(), anyhow::Error> {
    println!("Setting up workspace...");
    fs::copy("/generator/.", "../../..")?;
    Ok(())
}

#[allow(unused)]
pub fn debug_output() -> anyhow::Result<(), anyhow::Error> {
    println!("Working dir: {}", std::env::current_dir()?.display());
    println!("================================================");
    for entry in fs::read_dir("../../..")? {
        let entry = entry?;
        println!("{:?}", entry.path());
    }
    println!("================================================");
    Ok(())
}

#[allow(unused)]
pub fn sops_config_file_exists() -> anyhow::Result<bool, anyhow::Error> {
    let path = Path::new("actions/generator/workspace/.sops.yaml");
    Ok(path.exists())
}

#[allow(unused)]
pub fn create_default_sops_config_file() -> anyhow::Result<(), anyhow::Error> {
    println!("Creating .sops.yaml...");
    let team_key_fpr = get_key_fingerprint("team_private_key")?;
    let content = format!(
        "creation_rules:\n- key_groups:\n  - pgp:\n    - {}",
        team_key_fpr
    );
    fs::write("actions/generator/workspace/.sops.yaml", content)?;
    Ok(())
}

#[allow(unused)]
pub fn public_keys_provided() -> anyhow::Result<bool, anyhow::Error> {
    let public_keys_file = "actions/generator/public_keys.txt";
    let content = fs::read_to_string(public_keys_file).unwrap_or_default();
    let public_keys: Vec<&str> = content.lines().collect();
    Ok(!public_keys.is_empty())
}

#[allow(unused)]
pub fn update_sops_configuration_file() -> anyhow::Result<(), anyhow::Error> {
    println!("Updating .sops.yaml file with public keys...");
    let public_keys_file = "actions/generator/public_keys.txt";
    let binding = fs::read_to_string(public_keys_file)?;
    let public_keys = binding.lines().collect::<Vec<_>>();

    let mut config = fs::read_to_string("actions/generator/workspace/.sops.yaml")?;
    for key in public_keys {
        let key_fingerprint = get_key_fingerprint(key)?;
        config.push_str(&format!("\n    - {}", key_fingerprint));
    }
    fs::write("actions/generator/workspace/.sops.yaml", config)?;
    Ok(())
}

#[allow(unused)]
pub fn find_secret_files(workspace: &str) -> anyhow::Result<Vec<String>, anyhow::Error> {
    let mut secret_files = Vec::new();
    for entry in glob(&format!("{}/**/*.yaml", workspace))? {
        let path = entry?;
        let content = fs::read_to_string(&path)?;
        if content.contains("sops:") {
            secret_files.push(path.to_string_lossy().to_string());
        }
    }
    Ok(secret_files)
}

#[allow(unused)]
pub fn update_secret_file(file_path: &str) -> anyhow::Result<(), anyhow::Error> {
    println!("Updating secret file: {}", file_path);
    if Path::new(file_path).exists() {
        println!("The secret file {} exists.", file_path);
        Command::new("sops")
            .args(["updatekeys", file_path, "--yes"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .with_context(|| format!("Failed to re-encrypt secret file: {}", file_path))?;
    } else {
        println!("Creating secret file: {}", file_path);
        create_secret_file(file_path)?;
    }
    Ok(())
}

#[allow(unused)]
pub fn create_secret_file(file_path: &str) -> anyhow::Result<(), anyhow::Error> {
    fs::create_dir_all(Path::new(file_path).parent().unwrap())?;
    let sops_config_path = "actions/generator/workspace/.sops.yaml";

    let mut child = Command::new("sops")
        .args(["--config", sops_config_path, "-e", "/dev/stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::from(fs::File::create(file_path)?))
        .spawn()
        .context("Failed to spawn sops process")?;

    if let Some(stdin) = child.stdin.as_mut() {
        writeln!(stdin, "key: value").context("Failed to write to stdin")?;
    } else {
        return Err(anyhow::anyhow!("Failed to open stdin for sops process"));
    }

    child.wait().context("Failed to wait for sops process")?;
    Ok(())
}

#[allow(unused)]
pub fn set_message() -> anyhow::Result<(), anyhow::Error> {
    let message = "encrypt sops secrets and update sops.yaml";
    println!("message={}", message);
    Ok(())
}
