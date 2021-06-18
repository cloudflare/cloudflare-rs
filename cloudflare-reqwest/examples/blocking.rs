use anyhow::Result;
use cloudflare::{Credentials, OrderDirection, RequestBuilder};
use cloudflare_reqwest::SendBlockingClientExt as _;
use reqwest::blocking::Client;
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

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// A zone is a domain name along with its subdomains and other identities
    #[structopt(name = "zone")]
    Zone {
        #[structopt(name = "zone_identifier")]
        zone_id: String,
    },
    /// DNS Records for a zone
    #[structopt(name = "dns")]
    Dns {
        #[structopt(name = "zone_identifier")]
        zone_id: String,
    },
    /// Create a TXT record for a zone
    #[structopt(name = "create_txt_record")]
    CreateTxtRecord {
        #[structopt(name = "zone_identifier")]
        zone_id: String,
        name: String,
        content: String,
    },
    /// Activate a Worker on a route
    #[structopt(name = "list_routes")]
    ListRoutes {
        #[structopt(name = "zone_identifier")]
        zone_id: String,
    },
    /// Activate a Worker on a route
    #[structopt(name = "create_route")]
    CreateRoute {
        #[structopt(name = "zone_identifier")]
        zone_id: String,
        route_pattern: String,
        script_name: Option<String>,
    },
    /// Activate a Worker on a route
    DeleteRoute {
        #[structopt(name = "zone_identifier")]
        zone_id: String,
        #[structopt(name = "route_identifier")]
        route_id: String,
    },
    ListAccounts,
}

fn main() -> Result<()> {
    let Cli {
        email,
        auth_key,
        auth_token,
        command,
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
    match command {
        Command::Zone { zone_id } => {
            let request = cloudflare::api::zones::Get { zone_id: &zone_id };
            let response = builder.build(&request).send(&client)?;
            println!("{:#?}", response);
        }
        Command::Dns { zone_id } => {
            let request = cloudflare::api::dns::List {
                zone_id: &zone_id,
                params: cloudflare::api::dns::ListParams {
                    direction: Some(OrderDirection::Ascending),
                    ..Default::default()
                },
            };
            let response = builder.build(&request).send(&client)?;
            println!("{:#?}", response);
        }
        Command::CreateTxtRecord {
            zone_id,
            name,
            content,
        } => {
            let request = cloudflare::api::dns::Create {
                zone_id: &zone_id,
                params: cloudflare::api::dns::CreateParams {
                    name,
                    content: cloudflare::api::dns::DnsContent::TXT { content },
                    priority: None,
                    proxied: None,
                    ttl: None,
                },
            };
            let response = builder.build(&request).send(&client)?;
            println!("{:#?}", response);
        }
        Command::ListRoutes { zone_id } => {
            let request = cloudflare::api::workers::routes::List { zone_id: &zone_id };
            let response = builder.build(&request).send(&client)?;
            println!("{:#?}", response);
        }
        Command::ListAccounts => {
            let request = cloudflare::api::accounts::List::default();
            let response = builder.build(&request).send(&client)?;
            println!("{:#?}", response);
        }
        Command::CreateRoute {
            zone_id,
            route_pattern,
            script_name,
        } => {
            let request = cloudflare::api::workers::routes::Create {
                zone_id: &zone_id,
                params: cloudflare::api::workers::routes::CreateParams {
                    pattern: route_pattern,
                    script: script_name,
                },
            };
            let response = builder.build(&request).send(&client)?;
            println!("{:#?}", response);
        }
        Command::DeleteRoute { zone_id, route_id } => {
            let request = cloudflare::api::workers::routes::Delete {
                zone_id: &zone_id,
                route_id: &route_id,
            };
            let response = builder.build(&request).send(&client)?;
            println!("{:#?}", response);
        }
    }

    Ok(())
}
