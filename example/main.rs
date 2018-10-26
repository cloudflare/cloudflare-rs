#[macro_use] extern crate maplit;
extern crate clap;
extern crate cloudflare;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use cloudflare::{APIClient, HTTPAPIClient};
use cloudflare::auth::Credentials;


type SectionFunction<APIClientType> = fn(&ArgMatches, &APIClientType);


struct Section<'a, APIClientType: APIClient> {
    args: Vec<Arg<'a, 'a>>,
    description: &'a str,
    function: SectionFunction<APIClientType>,
}

fn zone<APIClientType: APIClient>(arg_matches: &ArgMatches, api_client: &APIClientType) {
    let response = api_client.zone_details(arg_matches.value_of("zone_identifier").unwrap());
    match response {
        Ok(success) => println!("Success: {:?}", success),
        Err(error) => println!("Error: {:?}", error),
    }
}

fn dns<APIClientType: APIClient>(arg_matches: &ArgMatches, api_client: &APIClientType) {
    let response = api_client.list_dns_records(arg_matches.value_of("zone_identifier").unwrap(), None);
    match response {
        Ok(success) => println!("Success: {:?}", success),
        Err(error) => println!("Error: {:?}", error),
    }
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
    let matched_sections = sections.iter()
        .filter(|&(section_name, _): &(&&str, &Section<HTTPAPIClient>)| matches.subcommand_matches(section_name).is_some());

    let key = matches.value_of("auth-key").unwrap();
    let email = matches.value_of("email").unwrap();

    let api_client = HTTPAPIClient::new(Credentials::User{
        key: key.to_string(),
        email: email.to_string(),
    });

    println!("{:?}", matches);
    for (section_name, section) in matched_sections {
        (section.function)(&matches.subcommand_matches(section_name).unwrap(), &api_client);
    }

    Ok(())
}
