use anyhow::{Context, Result, bail};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::process;

const KEYRING_SERVICE: &str = "linear-skill";
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

    /// Execute a GraphQL query against Linear's API.
    Query {
        /// GraphQL query string (plain text)
        #[arg(long, group = "query_input")]
        query: Option<String>,

        /// GraphQL query as a base64-encoded string (shell-safe)
        #[arg(long, group = "query_input")]
        query_base64: Option<String>,

        /// Optional JSON variables for the query
        #[arg(long)]
        variables: Option<String>,
    },
}

fn cmd_auth() -> Result<()> {
    eprintln!("Enter your Linear API key (input is hidden):");
    let key = rpassword::read_password().context("Failed to read API key")?;
    let key = key.trim();
    if key.is_empty() {
        bail!("API key cannot be empty");
    }
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .context("Failed to create keyring entry")?;
    entry
        .set_password(key)
        .context("Failed to store API key in keychain")?;
    eprintln!("API key stored successfully.");
    Ok(())
}

fn cmd_query(query: &str, variables: Option<&str>) -> Result<()> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .context("Failed to access keyring")?;
    let api_key = entry
        .get_password()
        .context("No API key found. Run `linear-skill auth` first to store your key.")?;

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
        Command::Query {
            query,
            query_base64,
            variables,
        } => {
            let query_str = match (query, query_base64) {
                (Some(q), _) => q.clone(),
                (_, Some(b64)) => {
                    let bytes = STANDARD
                        .decode(b64)
                        .expect("Invalid base64 in --query-base64");
                    String::from_utf8(bytes).expect("Query is not valid UTF-8")
                }
                _ => {
                    eprintln!("Error: either --query or --query-base64 is required");
                    process::exit(1);
                }
            };
            cmd_query(&query_str, variables.as_deref())
        }
    };
    if let Err(e) = result {
        eprintln!("Error: {e:#}");
        process::exit(1);
    }
}
