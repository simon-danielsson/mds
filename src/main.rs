use crate::utils::arg::Arguments;

mod constants;
mod parse;
mod utils;

struct Mds {
    a: Arguments,
}

impl Mds {
    fn new() -> anyhow::Result<Self> {
        let a = utils::arg::parse_arguments()?;
        Ok(Self { a })
    }
}

fn main() -> anyhow::Result<()> {
    let mds = Mds::new()?;

    if mds.a.help {
        println!("{}", constants::HELP_CONTENTS);
        return Ok(());
    }

    if mds.a.version {
        println!(
            "{p} v{v}",
            p = constants::APP_NAME,
            v = constants::APP_VERSION
        );
    }

    let s = std::fs::read_to_string(&mds.a.ip_path)?;
    let slideshow = parse::md_parse(s)?;

    std::fs::write("parser_output.txt", format!("{:#?}", slideshow))?;

    Ok(())
}
