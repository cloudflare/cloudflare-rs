#[macro_use]
extern crate maplit;
extern crate clap;
extern crate cloudflare;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use cloudflare::endpoints::{dns, zone};
use cloudflare::framework::{
    apiclient::ApiClient,
    auth::Credentials,
    mock::{MockApiClient, NoopEndpoint},
    response::{ApiFailure, ApiResponse, ApiResult},
    HttpApiClient, HttpApiClientConfig, OrderDirection,
};

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
    };

    let mut cli = App::new("cloudflare-rust")
        .version("0.0")
        .author("Argo Tunnel team <argo-tunnel-team@cloudflare.com>")
        .about("Issues example requests to thcliente Cloudflare API using the cloudflare-rust client library")
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

    let api_client = HttpApiClient::new(credentials, HttpApiClientConfig::default())?;

    for (section_name, section) in matched_sections {
        (section.function)(
            &matches.subcommand_matches(section_name).unwrap(),
            &api_client,
        );
    }

    Ok(())
}
