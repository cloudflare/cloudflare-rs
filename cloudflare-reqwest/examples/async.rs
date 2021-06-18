use anyhow::{Context, Result};
use cloudflare::{Credentials, JsonResponse, JsonResult, RequestBuilder};
use cloudflare_reqwest::SendClientExt as _;
use reqwest::Client;
use std::net::Ipv4Addr;
use structopt::StructOpt;

/// Issues example requests to the Cloudflare API using the cloudflare-rs client library.
#[derive(StructOpt)]
pub struct Cli {
    /// Email address associated with your account
    #[structopt(requires = "auth-key", env = "CF_RS_AUTH_KEY")]
    pub email: Option<String>,
    #[structopt(requires = "email")]
    pub auth_key: Option<String>,
    #[structopt(conflicts_with_all(&["email", "auth-key"]), env = "CF_RS_AUTH_TOKEN")]
    pub auth_token: Option<String>,
    pub account_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Cli {
        email,
        auth_key,
        auth_token,
        account_id,
    } = Cli::from_args();

    let credentials = match (email, auth_key, auth_token) {
        (Some(email), Some(auth_key), None) => Credentials::UserAuthKey {
            email,
            key: auth_key,
        },
        (None, None, Some(auth_token)) => Credentials::UserAuthToken(auth_token),
        (None, None, None) => {
            anyhow::bail!("No authentication provided!")
        }
        _ => anyhow::bail!("Unknown credential combination"),
    };
    let builder = RequestBuilder::new(credentials);

    let client = Client::new();

    use cloudflare::api::load_balancer;

    let request = load_balancer::pools::Create {
        account_id: &account_id,
        params: load_balancer::pools::CreateParams {
            name: "test-pool".to_owned(),
            description: Some("test description".to_owned()),
            enabled: Some(true),
            minimum_origins: Some(2),
            monitor: Some("9004c07f1c0f33255410e45590251cf4".to_owned()),
            notification_email: Some("test@example.com".to_owned()),
            origins: vec![
                load_balancer::pools::Origin {
                    name: "test-origin".to_owned(),
                    address: Ipv4Addr::new(152, 122, 3, 1).into(),
                    enabled: true,
                    weight: 1.0,
                },
                load_balancer::pools::Origin {
                    name: "test-origin-2".to_owned(),
                    address: Ipv4Addr::new(152, 122, 3, 2).into(),
                    enabled: true,
                    weight: 1.0,
                },
            ],
        },
    };
    let response = builder
        .build(&request)
        .send(&client)
        .await
        .context("Failed to create pool")?;
    let pool = match response {
        JsonResponse {
            body:
                JsonResult {
                    success: true,
                    result: Some(pool),
                    ..
                },
            ..
        } => pool,
        response => anyhow::bail!("Failed to create pool: {:#?}", response),
    };

    let request = load_balancer::pools::Get {
        account_id: &account_id,
        pool_id: &pool.id,
    };
    let details = builder
        .build(&request)
        .send(&client)
        .await
        .context("Failed to fetch pool")?;

    let request = load_balancer::pools::Delete {
        account_id: &account_id,
        pool_id: &pool.id,
    };
    let _ = builder
        .build(&request)
        .send(&client)
        .await
        .context("Failed to delete pool")?;
    let details_pool = match details {
        JsonResponse {
            body:
                JsonResult {
                    success: true,
                    result: Some(pool),
                    ..
                },
            ..
        } => pool,
        response => anyhow::bail!("Failed to create pool: {:#?}", response),
    };

    assert_eq!(pool, details_pool);

    Ok(())
}
