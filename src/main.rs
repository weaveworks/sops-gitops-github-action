use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use sops_gitops_github_action::import_gpg_key;
use sops_gitops_github_action::update_sops_config;

/// CLI arguments for the sops-gitops-github-action
#[derive(Debug, Parser)]
#[command(name = "sops-gitops-github-action1")]
struct MyArgs {
    /// The base64-encoded private GPG key
    #[arg(long)]
    private_key: String,

    /// A comma-separated list of base64-encoded public GPG keys
    #[arg(long)]
    public_keys: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = MyArgs::parse();

    // Step 1: Decode and import the private key
    println!("Importing private key...");
    import_gpg_key(&args.private_key)?;

    // Step 2: Decode and import public keys
    println!("Importing public keys...");
    for public_key in args.public_keys.split(',') {
        import_gpg_key(public_key)?;
    }

    // Step 3: Create or update .sops.yaml
    println!("Updating .sops.yaml...");
    update_sops_config(&args.public_keys)?;

    // Step 4: Encrypt/decrypt files (placeholder for actual file handling logic)
    println!("Action completed!");
    Ok(())
}
