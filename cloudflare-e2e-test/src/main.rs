#![forbid(unsafe_code)]

use clap::{Arg, Command};
use cloudflare::framework::async_api::Client as AsyncClient;
use cloudflare::framework::{async_api, auth::Credentials, Environment, HttpApiClientConfig};
use std::fmt::Display;
use std::net::{IpAddr, Ipv4Addr};

async fn tests(api_client: &AsyncClient, account_id: &str) -> anyhow::Result<()> {
    test_lb_pool(api_client, account_id).await?;
    println!("Tests passed");
    Ok(())
}

async fn test_lb_pool(api_client: &AsyncClient, account_identifier: &str) -> anyhow::Result<()> {
    use cloudflare::endpoints::load_balancing::*;

    // Create a pool
    let origins = vec![
        Origin {
            name: "test-origin".to_owned(),
            address: IpAddr::V4(Ipv4Addr::new(152, 122, 3, 1)),
            enabled: true,
            weight: 1.0,
        },
        Origin {
            name: "test-origin-2".to_owned(),
            address: IpAddr::V4(Ipv4Addr::new(152, 122, 3, 2)),
            enabled: true,
            weight: 1.0,
        },
    ];
    let pool = api_client
        .request(&create_pool::CreatePool {
            account_identifier,
            params: create_pool::Params {
                name: "test-pool",
                optional_params: Some(create_pool::OptionalParams {
                    description: Some("test description"),
                    enabled: Some(true),
                    minimum_origins: Some(2),
                    monitor: Some("9004c07f1c0f33255410e45590251cf4"),
                    notification_email: Some("test@example.com"),
                }),
                origins: &origins,
            },
        })
        .await
        .log_err(|e| println!("Error in CreatePool: {e}"))?
        .result;

    // Get the details, but wait until after we delete the pool to validate it.
    let pool_details = api_client
        .request(&pool_details::PoolDetails {
            account_identifier,
            identifier: &pool.id,
        })
        .await
        .log_err(|e| println!("Error in PoolDetails: {e}"));

    // Delete the pool
    let _ = api_client
        .request(&delete_pool::DeletePool {
            account_identifier,
            identifier: &pool.id,
        })
        .await
        .log_err(|e| println!("Error in DeletePool: {e}"))?;

    // Validate the pool we got was the same as the pool we sent
    let pool_details = pool_details?.result;
    assert_eq!(pool, pool_details);

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
    let email = matches.remove_one("email").unwrap();
    let key = matches.remove_one("auth-key");
    let token = matches.remove_one("auth-token");
    let account_id = matches
        .remove_one("account-id")
        .expect("account_id is mandatory");

    let credentials: Credentials = if let Some(key) = key {
        Credentials::UserAuthKey { email, key }
    } else if let Some(token) = token {
        Credentials::UserAuthToken { token }
    } else {
        panic!("Either API token or API key + email pair must be provided")
    };

    let api_client = async_api::Client::new(
        credentials,
        HttpApiClientConfig::default(),
        Environment::Production,
    )?;

    tests(&api_client, account_id).await
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
