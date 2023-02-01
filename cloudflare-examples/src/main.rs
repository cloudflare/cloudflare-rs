#![forbid(unsafe_code)]

use std::collections::HashMap;

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use cloudflare::endpoints::{account, dns, workers, zone};
use cloudflare::framework::endpoint::Endpoint;
use cloudflare::framework::response::{ApiError, ApiErrors};
use cloudflare::framework::{
    auth::Credentials,
    response::{ApiFailure, ApiResponse, ApiResult},
    Environment, HttpApiClient, HttpApiClientConfig, OrderDirection,
};
use serde::Serialize;

type SectionFunction<ApiClientType> = fn(&ArgMatches, &ApiClientType);

struct Section<'a> {
    args: Vec<Arg>,
    description: &'a str,
    function: SectionFunction<HttpApiClient>,
}

fn print_response<T: ApiResult>(response: ApiResponse<T>) {
    match response {
        Ok(success) => println!("Success: {success:#?}"),
        Err(e) => match e {
            ApiFailure::Error(status, errors) => {
                println!("HTTP {status}:");
                for err in errors.errors {
                    println!("Error {}: {}", err.code, err.message);
                    for (k, v) in err.other {
                        println!("{k}: {v}");
                    }
                }
                for (k, v) in errors.other {
                    println!("{k}: {v}");
                }
            }
            ApiFailure::Invalid(reqwest_err) => println!("Error: {reqwest_err}"),
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
                println!("HTTP {status}:");
                for err in errors.errors {
                    println!("Error {}: {}", err.code, err.message);
                    for (k, v) in err.other {
                        println!("{k}: {v}");
                    }
                }
                for (k, v) in errors.other {
                    println!("{k}: {v}");
                }
            }
            ApiFailure::Invalid(reqwest_err) => println!("Error: {reqwest_err}"),
        },
    }
}

fn zone(arg_matches: &ArgMatches, api_client: &HttpApiClient) {
    let zone_identifier = arg_matches.get_one::<String>("zone_identifier");
    let endpoint = zone::ZoneDetails {
        identifier: zone_identifier.unwrap(),
    };
    if api_client.is_mock() {
        add_static_mock(&endpoint);
    }
    let response = api_client.request(&endpoint);
    print_response(response)
}

fn dns(arg_matches: &ArgMatches, api_client: &HttpApiClient) {
    let zone_identifier = arg_matches.get_one::<String>("zone_identifier").unwrap();
    let endpoint = dns::ListDnsRecords {
        zone_identifier,
        params: dns::ListDnsRecordsParams {
            direction: Some(OrderDirection::Ascending),
            ..Default::default()
        },
    };
    if api_client.is_mock() {
        add_static_mock(&endpoint);
    }
    let response = api_client.request(&endpoint);

    print_response(response);
}

fn create_txt_record(arg_matches: &ArgMatches, api_client: &HttpApiClient) {
    let usage = "usage: create_txt_record ZONE_ID NAME CONTENT";

    let zone_id_missing = format!("missing '{}': {}", "ZONE_ID", usage);
    let zone_identifier = arg_matches
        .get_one::<String>("zone_identifier")
        .expect(&zone_id_missing);

    let name_missing = format!("missing '{}': {}", "NAME", usage);
    let name = arg_matches.get_one::<String>("name").expect(&name_missing);

    let content_missing = format!("missing '{}': {}", "CONTENT", usage);
    let content = arg_matches
        .get_one::<String>("content")
        .expect(&content_missing);

    let endpoint = dns::CreateDnsRecord {
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
    };
    if api_client.is_mock() {
        add_static_mock(&endpoint);
    }
    let response = api_client.request(&endpoint);

    print_response(response);
}

fn list_routes(arg_matches: &ArgMatches, api_client: &HttpApiClient) {
    let usage = "usage: list_routes ZONE_ID";

    let zone_id_missing = format!("missing '{}': {}", "ZONE_ID", usage);
    let zone_identifier = arg_matches
        .get_one::<String>("zone_identifier")
        .expect(&zone_id_missing);

    let endpoint = workers::ListRoutes { zone_identifier };
    if api_client.is_mock() {
        add_static_mock(&endpoint);
    }
    let response = api_client.request(&endpoint);

    print_response_json(response);
}

fn list_accounts(_arg_matches: &ArgMatches, api_client: &HttpApiClient) {
    let endpoint = account::ListAccounts { params: None };
    if api_client.is_mock() {
        add_static_mock(&endpoint);
    }
    let response = api_client.request(&endpoint);

    print_response_json(response);
}

fn create_route(arg_matches: &ArgMatches, api_client: &HttpApiClient) {
    let usage = "usage: create_route ZONE_ID SCRIPT_NAME ROUTE_PATTERN";

    let zone_id_missing = format!("missing '{}': {}", "ZONE_ID", usage);
    let zone_identifier = arg_matches
        .get_one::<String>("zone_identifier")
        .expect(&zone_id_missing);

    let route_pattern_missing = format!("missing '{}': {}", "ROUTE_PATTERN", usage);
    let route_pattern = arg_matches
        .get_one::<String>("route_pattern")
        .expect(&route_pattern_missing);

    let script_name = arg_matches.get_one::<String>("script_name");

    let endpoint = workers::CreateRoute {
        zone_identifier,
        params: workers::CreateRouteParams {
            pattern: route_pattern.to_string(),
            script: script_name.map(|n| n.to_string()),
        },
    };
    if api_client.is_mock() {
        add_static_mock(&endpoint);
    }
    let response = api_client.request(&endpoint);

    print_response_json(response);
}

fn delete_route(arg_matches: &ArgMatches, api_client: &HttpApiClient) {
    let usage = "usage: delete_route ZONE_ID ROUTE_ID";

    let zone_id_missing = format!("missing '{}': {}", "ZONE_ID", usage);
    let zone_identifier = arg_matches
        .get_one::<String>("zone_identifier")
        .expect(&zone_id_missing);

    let route_id_missing = format!("missing '{}': {}", "ROUTE_PATTERN", usage);
    let route_id = arg_matches
        .get_one::<String>("route_identifier")
        .expect(&route_id_missing);

    let endpoint = workers::DeleteRoute {
        zone_identifier,
        identifier: route_id,
    };
    if api_client.is_mock() {
        add_static_mock(&endpoint);
    }
    let response = api_client.request(&endpoint);

    print_response_json(response);
}

/// Add and leak a mock (so it runs for 'static)
fn add_static_mock<ResultType>(endpoint: &dyn Endpoint<ResultType>)
where
    ResultType: ApiResult,
{
    let body = ApiErrors {
        errors: vec![ApiError {
            code: 9999,
            message: "This is a mocked failure response".to_owned(),
            other: HashMap::new(),
        }],
        other: HashMap::new(),
    };

    let m = Box::new(
        mockito::mock(
            endpoint.method().as_str(),
            format!("/{}", endpoint.path()).as_str(),
        )
        .with_status(500)
        .with_body(serde_json::to_string(&body).unwrap())
        .create(),
    );
    Box::leak(m);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sections = HashMap::from([
        (
            "zone",
            Section {
                args: vec![Arg::new("zone_identifier").required(true)],
                description:
                    "A Zone is a domain name along with its subdomains and other identities",
                function: zone,
            },
        ),
        (
            "dns",
            Section {
                args: vec![Arg::new("zone_identifier").required(true)],
                description: "DNS Records for a Zone",
                function: dns,
            },
        ),
        (
            "create_txt_record",
            Section {
                args: vec![
                    Arg::new("zone_identifier").required(true),
                    Arg::new("name").required(true),
                    Arg::new("content").required(true),
                ],
                description: "Create a TXT record for a zone",
                function: create_txt_record,
            },
        ),
        (
            "list_routes",
            Section {
                args: vec![Arg::new("zone_identifier").required(true)],
                description: "Activate a Worker on a Route",
                function: list_routes,
            },
        ),
        (
            "list_accounts",
            Section {
                args: vec![],
                description: "List accounts",
                function: list_accounts,
            },
        ),
        (
            "create_route",
            Section {
                args: vec![
                    Arg::new("zone_identifier").required(true),
                    Arg::new("route_pattern").required(true),
                    Arg::new("script_name").required(false),
                ],
                description: "Activate a Worker on a Route",
                function: create_route,
            },
        ),
        (
            "delete_route",
            Section {
                args: vec![
                    Arg::new("zone_identifier").required(true),
                    Arg::new("route_identifier").required(true),
                ],
                description: "Activate a Worker on a Route",
                function: delete_route,
            },
        ),
    ]);

    let mut cli = Command::new("cloudflare-rust")
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
        .arg(Arg::new("mock")
            .long("mock")
            .help("Run this request against the mock API endpoint")
            .action(ArgAction::SetTrue))
        .group(ArgGroup::new("auth")
            .args(["email", "auth-key", "auth-token"])
            .multiple(true)
            .required(true))
        .arg_required_else_help(true);

    for (section_name, section) in sections.iter() {
        let mut subcommand = Command::new(*section_name).about(section.description);

        for arg in &section.args {
            subcommand = subcommand.arg(arg);
        }
        cli = cli.subcommand(subcommand);
    }

    let mut matches = cli.get_matches();
    let email = matches.remove_one("email");
    let key = matches.remove_one("auth-key");
    let token = matches.remove_one("auth-token");
    let environment = if matches.get_flag("mock") {
        Environment::Mockito
    } else {
        Environment::Production
    };

    let matched_sections = sections
        .iter()
        .filter(|&(section_name, _): &(&&str, &Section<'_>)| {
            matches.subcommand_matches(section_name).is_some()
        });

    let credentials: Credentials = if let Some(key) = key {
        Credentials::UserAuthKey {
            email: email.unwrap(),
            key,
        }
    } else if let Some(token) = token {
        Credentials::UserAuthToken { token }
    } else {
        panic!("Either API token or API key + email pair must be provided")
    };

    let api_client = HttpApiClient::new(credentials, HttpApiClientConfig::default(), environment)?;

    for (section_name, section) in matched_sections {
        (section.function)(
            matches.subcommand_matches(section_name).unwrap(),
            &api_client,
        );
    }

    Ok(())
}
