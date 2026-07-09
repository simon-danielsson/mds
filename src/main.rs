mod constants;
mod utils;

fn main() -> anyhow::Result<()> {
    let a = utils::arg::parse_arguments()?;

    if a.help {
        println!("{}", constants::HELP_CONTENTS);
        return Ok(());
    }
    if a.version {
        println!(
            "{p} v{v}",
            p = constants::APP_NAME,
            v = constants::APP_VERSION
        );
    }

    // let s = std::fs::read_to_string(&a.ip_path)?;
    // println!("{}", s);
    println!("{:#?}", a);
    Ok(())
}
