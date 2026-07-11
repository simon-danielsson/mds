use std::path::PathBuf;

use anyhow::anyhow;

use crate::constants::APP_NAME;

#[derive(Default, Debug)]
pub struct Arguments {
    pub help: bool,
    pub version: bool,
    pub op_path: Option<PathBuf>,
    pub ip_path: PathBuf,
}

pub fn parse_arguments() -> anyhow::Result<Arguments> {
    let run_help_msg = format!("run \"{APP_NAME} -h\" for more information");
    let mut it = std::env::args();

    if it.len() < 2 {
        return Err(anyhow!("no argument was provided - {run_help_msg}"));
    }

    let mut a = Arguments::default();

    it.next(); // skip program name
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                a.help = true;
                return Ok(a);
            }

            "-v" | "--version" => {
                a.version = true;
                return Ok(a);
            }
            "-o" | "--output" => {
                let arg = it.next().ok_or_else(|| {
                    anyhow!("missing output path after {arg}")
                })?;
                a.op_path = Some(PathBuf::from(arg));
            }

            _ if arg.starts_with('-') => {
                return Err(anyhow!("unknown option \"{arg}\" - {run_help_msg}"));
            }

            _ => {
                let ip = PathBuf::from(&arg);
                if !ip.is_file() {
                    return Err(anyhow!(
                            "\"{arg}\" does not exist - {run_help_msg}"
                    ));
                }
                a.ip_path = ip;
            }
        }
    }

    if !a.help && !a.version && a.ip_path.as_os_str().is_empty() {
        return Err(anyhow!("no input file was provided - {run_help_msg}"));
    }

    Ok(a)
}
