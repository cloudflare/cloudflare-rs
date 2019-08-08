#[macro_use]
extern crate maplit;
extern crate clap;
extern crate cloudflare;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use cloudflare::apiclient::APIClient;
use cloudflare::auth::Credentials;
use cloudflare::dns;
use cloudflare::mock::{MockAPIClient, NoopEndpoint};
use cloudflare::response::{APIFailure, APIResponse, APIResult};
use cloudflare::zone;
use cloudflare::{HTTPAPIClient, OrderDirection};

type SectionFunction<APIClientType> = fn(&ArgMatches, &APIClientType);

struct Section<'a, APIClientType: APIClient> {
    args: Vec<Arg<'a, 'a>>,
    description: &'a str,
    function: SectionFunction<APIClientType>,
}

fn print_response<T: APIResult>(response: APIResponse<T>) {
    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => match e {
            APIFailure::Error(status, errors) => {
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
            APIFailure::Invalid(reqwest_err) => println!("Error: {}", reqwest_err),
        },
    }
}

fn zone<APIClientType: APIClient>(arg_matches: &ArgMatches, api_client: &APIClientType) {
    let zone_identifier = arg_matches.value_of("zone_identifier").unwrap();
    let response = api_client.request(&zone::ZoneDetails {
        identifier: zone_identifier,
    });
    print_response(response)
}

fn dns<APIClientType: APIClient>(arg_matches: &ArgMatches, api_client: &APIClientType) {
    let zone_identifier = arg_matches.value_of("zone_identifier").unwrap();
    let response = api_client.request(&dns::ListDNSRecords {
        zone_identifier,
        params: dns::ListDNSRecordsParams {
            direction: Some(OrderDirection::Ascending),
            ..Default::default()
        },
    });

    print_response(response);
}

fn create_txt_record<APIClientType: APIClient>(
    arg_matches: &ArgMatches,
    api_client: &APIClientType,
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

    let response = api_client.request(&dns::CreateDNSRecord {
        zone_identifier,
        params: dns::CreateDNSRecordParams {
            name,
            content: dns::DNSContent::TXT {
                content: content.to_owned(),
            },
            priority: None,
            proxied: None,
            ttl: None,
        },
    });

    print_response(response);
}

fn mock_api<APIClientType: APIClient>(_args: &ArgMatches, _api: &APIClientType) {
    let mock_api = MockAPIClient {};
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
        .about("Issues example requests to the Cloudflare API using the cloudflare-rust client library")
        .arg(Arg::with_name("email")
            .long("email")
            .help("Email address associated with your account")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("auth-key")
            .long("auth-key")
            .env("CF_RS_AUTH_KEY")
            .help("API key generated on the \"My Account\" page")
            .takes_value(true)
            .required(true))
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
            .filter(|&(section_name, _): &(&&str, &Section<HTTPAPIClient>)| {
                matches.subcommand_matches(section_name).is_some()
            });

    let key = matches.value_of("auth-key").unwrap();
    let email = matches.value_of("email").unwrap();

    let api_client = HTTPAPIClient::new(Credentials::User {
        key: key.to_string(),
        email: email.to_string(),
    });

    for (section_name, section) in matched_sections {
        (section.function)(
            &matches.subcommand_matches(section_name).unwrap(),
            &api_client,
        );
    }

    Ok(())
}
