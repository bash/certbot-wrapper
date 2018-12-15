use std::io;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;

pub(crate) fn run_certbot(config_file: PathBuf, additional_args: &[String]) -> io::Result<()> {
    const CERTBOT_EXECUTABLE: &str = "certbot";

    let exit_status = Command::new(CERTBOT_EXECUTABLE)
        .arg("--config")
        .arg(config_file)
        .args(additional_args)
        .status()?;

    if !exit_status.success() {
        if let Some(code) = exit_status.code() {
            eprintln!("certbot exited with non-zero exit code: {}", code);
            exit(code);
        } else {
            eprintln!("certbot was killed by signal");
            exit(1);
        }
    }

    Ok(())
}
