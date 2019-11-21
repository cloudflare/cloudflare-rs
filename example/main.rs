#[macro_use]
extern crate maplit;
extern crate clap;
extern crate cloudflare;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use cloudflare::endpoints::{dns, workers, zone};
use cloudflare::framework::{
    apiclient::ApiClient,
    auth::Credentials,
    mock::{MockApiClient, NoopEndpoint},
    response::{ApiFailure, ApiResponse, ApiResult},
    Environment, HttpApiClient, HttpApiClientConfig, OrderDirection,
};
use serde::Serialize;

type SectionFunction<ApiClientType> = fn(&ArgMatches, &ApiClientType);

struct Section<'a, ApiClientType: ApiClient> {
    args: Vec<Arg<'a, 'a>>,
    description: &'a str,
    function: SectionFunction<ApiClientType>,
}

fn print_response<T: ApiResult>(response: ApiResponse<T>) {
    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => match e {
            ApiFailure::Error(status, errors) => {
                println!("HTTP {}:", status);
                for err in errors.errors {
                    println!("Error {}: {}", err.code, err.message);
                    for (k, v) in err.other {
                        println!("{}: {}", k, v);
                    }
                }
                for (k, v) in errors.other {
                    println!("{}: {}", k, v);
                }
            }
            ApiFailure::Invalid(reqwest_err) => println!("Error: {}", reqwest_err),
        },
    }
}

/// Sometimes you want to pipe results to jq etc
fn print_response_json<T: ApiResult>(response: ApiResponse<T>)
where
    T: Serialize,
{
    match response {
        Ok(success) => println!("{}", serde_json::to_string(&success.result).unwrap()),
        Err(e) => match e {
            ApiFailure::Error(status, errors) => {
                println!("HTTP {}:", status);
                for err in errors.errors {
                    println!("Error {}: {}", err.code, err.message);
                    for (k, v) in err.other {
                        println!("{}: {}", k, v);
                    }
                }
                for (k, v) in errors.other {
                    println!("{}: {}", k, v);
                }
            }
            ApiFailure::Invalid(reqwest_err) => println!("Error: {}", reqwest_err),
        },
    }
}

fn zone<ApiClientType: ApiClient>(arg_matches: &ArgMatches, api_client: &ApiClientType) {
    let zone_identifier = arg_matches.value_of("zone_identifier").unwrap();
    let response = api_client.request(&zone::ZoneDetails {
        identifier: zone_identifier,
    });
    print_response(response)
}

fn dns<ApiClientType: ApiClient>(arg_matches: &ArgMatches, api_client: &ApiClientType) {
    let zone_identifier = arg_matches.value_of("zone_identifier").unwrap();
    let response = api_client.request(&dns::ListDnsRecords {
        zone_identifier,
        params: dns::ListDnsRecordsParams {
            direction: Some(OrderDirection::Ascending),
            ..Default::default()
        },
    });

    print_response(response);
}

fn create_txt_record<ApiClientType: ApiClient>(
    arg_matches: &ArgMatches,
    api_client: &ApiClientType,
) {
    let usage = "usage: create_txt_record ZONE_ID NAME CONTENT";

    let zone_id_missing = format!("missing '{}': {}", "ZONE_ID", usage);
    let zone_identifier = arg_matches
        .value_of("zone_identifier")
        .expect(&zone_id_missing);

    let name_missing = format!("missing '{}': {}", "NAME", usage);
    let name = arg_matches.value_of("name").expect(&name_missing);

    let content_missing = format!("missing '{}': {}", "CONTENT", usage);
    let content = arg_matches.value_of("content").expect(&content_missing);

    let response = api_client.request(&dns::CreateDnsRecord {
        zone_identifier,
        params: dns::CreateDnsRecordParams {
            name,
            content: dns::DnsContent::TXT {
                content: content.to_owned(),
            },
            priority: None,
            proxied: None,
            ttl: None,
        },
    });

    print_response(response);
}

fn list_routes<ApiClientType: ApiClient>(arg_matches: &ArgMatches, api_client: &ApiClientType) {
    let usage = "usage: list_routes ZONE_ID";

    let zone_id_missing = format!("missing '{}': {}", "ZONE_ID", usage);
    let zone_identifier = arg_matches
        .value_of("zone_identifier")
        .expect(&zone_id_missing);

    let response = api_client.request(&workers::ListRoutes { zone_identifier });

    print_response_json(response);
}

fn create_route<ApiClientType: ApiClient>(arg_matches: &ArgMatches, api_client: &ApiClientType) {
    let usage = "usage: create_route ZONE_ID SCRIPT_NAME ROUTE_PATTERN";

    let zone_id_missing = format!("missing '{}': {}", "ZONE_ID", usage);
    let zone_identifier = arg_matches
        .value_of("zone_identifier")
        .expect(&zone_id_missing);

    let route_pattern_missing = format!("missing '{}': {}", "ROUTE_PATTERN", usage);
    let route_pattern = arg_matches
        .value_of("route_pattern")
        .expect(&route_pattern_missing);

    let script_name = arg_matches.value_of("script_name");

    let response = api_client.request(&workers::CreateRoute {
        zone_identifier,
        params: workers::CreateRouteParams {
            pattern: route_pattern.to_string(),
            script: script_name.map(|n| n.to_string()),
        },
    });

    print_response_json(response);
}

fn delete_route<ApiClientType: ApiClient>(arg_matches: &ArgMatches, api_client: &ApiClientType) {
    let usage = "usage: delete_route ZONE_ID ROUTE_ID";

    let zone_id_missing = format!("missing '{}': {}", "ZONE_ID", usage);
    let zone_identifier = arg_matches
        .value_of("zone_identifier")
        .expect(&zone_id_missing);

    let route_id_missing = format!("missing '{}': {}", "ROUTE_PATTERN", usage);
    let route_id = arg_matches.value_of("route_id").expect(&route_id_missing);

    let response = api_client.request(&workers::DeleteRoute {
        zone_identifier,
        identifier: route_id,
    });

    print_response_json(response);
}

fn mock_api<ApiClientType: ApiClient>(_args: &ArgMatches, _api: &ApiClientType) {
    let mock_api = MockApiClient {};
    let endpoint = NoopEndpoint {};
    let _ = mock_api.request(&endpoint);
    println!("Ran mock API")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sections = hashmap! {
        "zone" => Section{
            args: vec![Arg::with_name("zone_identifier").required(true)],
            description: "A Zone is a domain name along with its subdomains and other identities",
            function: zone
        },
        "dns" => Section{
            args: vec![Arg::with_name("zone_identifier").required(true)],
            description: "DNS Records for a Zone",
            function: dns
        },
        "create_txt_record" => Section{
            args: vec![
                Arg::with_name("zone_identifier").required(true),
                Arg::with_name("name").required(true),
                Arg::with_name("content").required(true),
                ],
            description: "Create a TXT record for a zone",
            function: create_txt_record
        },
        "mock_api" => Section{
            args: vec![],
            description: "Run a mock API request",
            function: mock_api
        },
        "list_routes" => Section{
            args: vec![
                Arg::with_name("zone_identifier").required(true),
            ],
            description: "Activate a Worker on a Route",
            function: list_routes
        },
        "create_route" => Section{
            args: vec![
                Arg::with_name("zone_identifier").required(true),
                Arg::with_name("route_pattern").required(true),
                Arg::with_name("script_name").required(false),
            ],
            description: "Activate a Worker on a Route",
            function: create_route
        },
        "delete_route" => Section{
            args: vec![
                Arg::with_name("zone_identifier").required(true),
                Arg::with_name("route_identifier").required(true),
            ],
            description: "Activate a Worker on a Route",
            function: delete_route
        },
    };

    let mut cli = App::new("cloudflare-rust")
        .version("0.0")
        .author("Argo Tunnel team <argo-tunnel-team@cloudflare.com>")
        .about("Issues example requests to the Cloudflare API using the cloudflare-rust client library")
        .arg(Arg::with_name("email")
            .long("email")
            .help("Email address associated with your account")
            .takes_value(true)
            .requires("auth-key"))
        .arg(Arg::with_name("auth-key")
            .long("auth-key")
            .env("CF_RS_AUTH_KEY")
            .help("API key generated on the \"My Account\" page")
            .takes_value(true)
            .requires("email"))
        .arg(Arg::with_name("auth-token")
            .long("auth-token")
            .env("CF_RS_AUTH_TOKEN")
            .help("API token generated on the \"My Account\" page")
            .takes_value(true)
            .conflicts_with_all(&["email", "auth-key"]))
        .setting(AppSettings::ArgRequiredElseHelp);

    for (section_name, section) in sections.iter() {
        let mut subcommand = SubCommand::with_name(section_name).about(section.description);

        for arg in &section.args {
            subcommand = subcommand.arg(arg);
        }
        cli = cli.subcommand(subcommand);
    }

    let matches = cli.get_matches();
    let matched_sections =
        sections
            .iter()
            .filter(|&(section_name, _): &(&&str, &Section<HttpApiClient>)| {
                matches.subcommand_matches(section_name).is_some()
            });

    let email = matches.value_of("email");
    let key = matches.value_of("auth-key");
    let token = matches.value_of("auth-token");

    let credentials: Credentials = if let Some(key) = key {
        Credentials::UserAuthKey {
            email: email.unwrap().to_string(),
            key: key.to_string(),
        }
    } else if let Some(token) = token {
        Credentials::UserAuthToken {
            token: token.to_string(),
        }
    } else {
        panic!("Either API token or API key + email pair must be provided")
    };

    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        Environment::Production,
    )?;

    for (section_name, section) in matched_sections {
        (section.function)(
            &matches.subcommand_matches(section_name).unwrap(),
            &api_client,
        );
    }

    Ok(())
}
