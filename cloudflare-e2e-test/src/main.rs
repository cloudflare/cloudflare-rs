#![forbid(unsafe_code)]
mod routing_performance;
mod storage_databases;

use clap::{Arg, Command};
use cloudflare::framework::client::async_api::Client as AsyncClient;
use cloudflare::framework::client::ClientConfig;
use cloudflare::framework::{auth::Credentials, client::async_api, Environment};
use std::fmt::Display;

async fn tests(api_client: &AsyncClient, account_id: &str) -> anyhow::Result<()> {
    routing_performance::load_balancers::test_lb_pool(api_client, account_id).await?;
    storage_databases::kv::test_kv(api_client, account_id).await?;
    println!("Tests passed");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli =
    Command::new("Cloudflare-rs E2E tests")
        .version("0.0")
        .author("Argo Tunnel team <argo-tunnel-team@cloudflare.com>")
        .about("Issues example requests to the Cloudflare API using the cloudflare-rust client library")
        .arg(Arg::new("email")
            .long("email")
            .env("CF_RS_EMAIL")
            .help("Email address associated with your account")
            .requires("auth-key"))
        .arg(Arg::new("auth-key")
            .long("auth-key")
            .env("CF_RS_AUTH_KEY")
            .help("API key generated on the \"My Account\" page")
            .requires("email"))
        .arg(Arg::new("auth-token")
            .long("auth-token")
            .env("CF_RS_AUTH_TOKEN")
            .help("API token generated on the \"My Account\" page")
            .conflicts_with_all(["email", "auth-key"]))
        .arg(Arg::new("account-id")
            .long("account-id")
            .env("CF_RS_ZONE_ID")
            .help("The ID of the account tests should be run on"))
        .arg_required_else_help(true);

    let mut matches = cli.get_matches();
    let email = matches.remove_one("email");
    let key = matches.remove_one("auth-key");
    let token = matches.remove_one("auth-token");
    let account_id: String = matches
        .remove_one("account-id")
        .expect("account_id is mandatory");

    let credentials: Credentials = if let (Some(email), Some(key)) = (email, key) {
        Credentials::UserAuthKey { email, key }
    } else if let Some(token) = token {
        Credentials::UserAuthToken { token }
    } else {
        panic!("Either API token or API key + email pair must be provided")
    };

    let api_client = async_api::Client::new(
        credentials,
        ClientConfig::default(),
        Environment::Production,
    )?;

    tests(&api_client, account_id.as_str()).await
}

pub trait ResultExt<T, E: Display> {
    /// Convenience function for logging errors inside results.
    /// Basically just `map_err` except the closure argument doesn't return anything,
    /// and `.log_err` always returns `self`.
    fn log_err<L: FnOnce(&E)>(self, log: L) -> Self;
}

impl<T, E: Display> ResultExt<T, E> for Result<T, E> {
    fn log_err<L: FnOnce(&E)>(self, log: L) -> Self {
        if let Err(e) = &self {
            log(e)
        }
        self
    }
}
