use self::certbot::run_certbot;
use self::config::get_config_files;
use std::io;

mod certbot;
mod config;

pub struct Arguments {
    pub config_path: String,
    pub certbot_args: Vec<String>,
}

pub fn renew_certificates(arguments: Arguments) -> io::Result<()> {
    let config_files = get_config_files(&arguments.config_path)?;

    for config_file in config_files {
        let config_file = config_file?;
        let path = config_file.path();
        run_certbot(path, &arguments.certbot_args)?;
    }

    Ok(())
}
