use std::path::PathBuf;

use crate::utils::arg::Arguments;

mod constants;
mod gen_html;
mod parse;
mod statics;
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
        return Ok(());
    }

    let s = std::fs::read_to_string(&mds.a.ip_path)?;

    let slideshow = parse::md_parse(s)?;

    // write output to debug log
    // std::fs::write("parser_output.txt", format!("{:#?}", &slideshow))?;

    let html = gen_html::generate(slideshow)?;
    output_html(mds.a, html)?;

    Ok(())
}

fn output_html(prog_args: Arguments, content: String) -> anyhow::Result<()> {
    let mut output_path: PathBuf = {
        let mut name = prog_args.ip_path.clone();
        name.set_extension("html");
        name
    };

    if let Some(op) = prog_args.op_path {
        output_path = op;
    }

    std::fs::write(output_path, content)?;
    Ok(())
}
