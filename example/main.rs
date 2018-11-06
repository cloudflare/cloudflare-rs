#[macro_use]
extern crate maplit;
extern crate clap;
extern crate cloudflare;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use cloudflare::auth::Credentials;
use cloudflare::dns;
use cloudflare::response::{APIFailure, APIResponse, APIResult};
use cloudflare::zone;
use cloudflare::{APIClient, HTTPAPIClient, OrderDirection};

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
                println!("Error {}:", status);
                for err in errors {
                    println!("Error {}: {}", err.code, err.message);
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
        zone_identifier: zone_identifier,
        params: dns::ListDNSRecordsParams {
            direction: Some(OrderDirection::Ascending),
            ..Default::default()
        },
    });

    print_response(response);
}

fn main() -> Result<(), Box<std::error::Error>> {
    let sections = hashmap!{
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
