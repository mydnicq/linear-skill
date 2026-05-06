use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::env;
use std::fs;
use std::process;
use uuid::Uuid;

const KEYRING_USER: &str = "api-key";
const LINEAR_API_URL: &str = "https://api.linear.app/graphql";

#[derive(Parser)]
#[command(name = "linear-skill", about = "Query Linear's GraphQL API")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Store or update your Linear API key in the OS keychain.
    #[command(
        long_about = "Store or update your Linear API key in the OS keychain.\n\n\
        WARNING: This command is for human users only. AI agents must NEVER run this command."
    )]
    Auth,

    /// Print a unique file path in the OS temp directory for writing query files.
    TempPath,

    /// Execute a GraphQL query against Linear's API.
    Query {
        /// Path to a file containing the GraphQL query (and optional variables separated by `---`)
        #[arg(long)]
        query_file: String,
    },
}

/// Returns a project-scoped keyring service name: `"linear-skill:{canonical_binary_dir}"`.
/// Using the path in the service field makes each entry visually distinct in Keychain Access.
fn keyring_service() -> Result<String> {
    let exe = env::current_exe().context("Failed to determine binary location")?;
    let dir = exe.parent().context("Binary has no parent directory")?;
    let path = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    Ok(format!("linear-skill:{}", path.display()))
}

fn cmd_auth() -> Result<()> {
    let service = keyring_service()?;
    eprintln!("Enter your Linear API key (input is hidden):");
    let key = rpassword::read_password().context("Failed to read API key")?;
    let key = key.trim();
    if key.is_empty() {
        bail!("API key cannot be empty");
    }
    let entry =
        keyring::Entry::new(&service, KEYRING_USER).context("Failed to create keyring entry")?;
    entry
        .set_password(key)
        .context("Failed to store API key in keychain")?;
    let exe = env::current_exe().unwrap_or_default();
    let dir = exe.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    println!(
        "API key stored for project directory: {}",
        dir.canonicalize().unwrap_or(dir).display()
    );
    Ok(())
}

fn cmd_query(query: &str, variables: Option<&str>) -> Result<()> {
    let service = keyring_service()?;
    let entry =
        keyring::Entry::new(&service, KEYRING_USER).context("Failed to access keyring")?;
    let api_key = entry.get_password().context(
        "No API key found for this project directory. Run `linear-skill auth` from this directory first.",
    )?;

    let mut body = serde_json::json!({ "query": query });
    if let Some(vars) = variables {
        let parsed: Value =
            serde_json::from_str(vars).context("Failed to parse --variables as JSON")?;
        body["variables"] = parsed;
    }

    let agent = ureq::Agent::new_with_config(
        ureq::config::Config::builder()
            .http_status_as_error(false)
            .build(),
    );

    let mut response = agent
        .post(LINEAR_API_URL)
        .header("Authorization", &api_key)
        .header("Content-Type", "application/json")
        .send_json(&body)
        .context("Failed to send request to Linear API")?;

    let status = response.status();
    let json: Value = response
        .body_mut()
        .read_json()
        .context("Failed to read response as JSON")?;

    // Print the full JSON to stdout regardless of status (agents need the error details too)
    println!("{}", serde_json::to_string(&json)?);

    // Surface errors to stderr so agents get clear feedback
    if !status.is_success() {
        if let Some(errors) = json.get("errors") {
            eprintln!("GraphQL errors (HTTP {}):", status.as_u16());
            if let Some(arr) = errors.as_array() {
                for err in arr {
                    if let Some(msg) = err.get("message").and_then(|m| m.as_str()) {
                        eprintln!("  - {msg}");
                    }
                }
            }
        } else {
            eprintln!("HTTP error: {}", status.as_u16());
        }
        process::exit(1);
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let result = match &cli.command {
        Command::Auth => cmd_auth(),
        Command::TempPath => {
            let dir = env::temp_dir();
            let path = dir.join(format!("linear-skill-{}.graphql", Uuid::new_v4()));
            println!("{}", path.display());
            return;
        }
        Command::Query { query_file } => {
            let contents = match fs::read_to_string(query_file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: Failed to read query file {query_file}: {e}");
                    process::exit(1);
                }
            };

            let (query_str, variables): (String, Option<String>) = match contents.split_once("\n---\n") {
                Some((q, v)) => {
                    let vars = if v.trim().is_empty() {
                        None
                    } else {
                        Some(v.trim().to_owned())
                    };
                    (q.trim().to_owned(), vars)
                }
                None => (contents.trim().to_owned(), None),
            };

            if query_str.is_empty() {
                eprintln!("Error: Query file is empty or contains only a separator");
                process::exit(1);
            }

            cmd_query(&query_str, variables.as_deref())
        }
    };
    if let Err(e) = result {
        eprintln!("Error: {e:#}");
        process::exit(1);
    }
}
