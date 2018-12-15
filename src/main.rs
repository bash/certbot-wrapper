#[macro_use]
extern crate clap as clap;

use clap::Arg;
use renew_letsencrypt_certs::{renew_certificates, Arguments};
use std::io;

fn parse_arguments() -> Arguments {
    const CONFIG_PATH_ARG: &str = "CONFIG_PATH";
    const CERTBOT_ARGS: &str = "CERTBOT_ARGS";
    const DEFAULT_CONFIG_PATH: &str = "/etc/letsencrypt/conf.d";

    let matches = app_from_crate!()
        .arg(
            Arg::with_name(CONFIG_PATH_ARG)
                .long("config-path")
                .takes_value(true)
                .value_name("CONFIG_PATH"),
        )
        .arg(
            Arg::with_name(CERTBOT_ARGS)
                .help("Additional arguments to pass directly to certbot")
                .raw(true),
        )
        .get_matches();

    let config_path = matches
        .value_of(CONFIG_PATH_ARG)
        .unwrap_or(DEFAULT_CONFIG_PATH)
        .to_owned();

    let certbot_args: Vec<_> = matches
        .values_of(CERTBOT_ARGS)
        .map(|values| values.map(String::from).collect())
        .unwrap_or_default();

    Arguments {
        config_path,
        certbot_args,
    }
}

fn main() -> io::Result<()> {
    let args = parse_arguments();

    renew_certificates(args)
}
