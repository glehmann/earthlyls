use clap::Parser;
use xshell::{cmd, Shell};

/// Utility commands
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
#[command(name = "cargo xtask")]
#[command(bin_name = "cargo xtask")]
enum Command {
    /// Generate the earthly command description files
    GenerateDescriptions,
}

fn main() -> anyhow::Result<()> {
    let args = Command::parse();
    match args {
        Command::GenerateDescriptions => generate_descriptions()?,
    };
    Ok(())
}

const EARTHFILE_MD: &str =
    "https://raw.githubusercontent.com/earthly/earthly/main/docs/earthfile/earthfile.md";

fn generate_descriptions() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let src_dir = sh.current_dir();
    let tmp_dir = sh.create_temp_dir()?;
    sh.change_dir(tmp_dir.path());
    cmd!(sh, "wget {EARTHFILE_MD}").run()?;
    cmd!(sh, "mdsplit -o doc -l2 earthfile.md").run()?;
    let docs = cmd!(sh, "find doc/Earthfile-reference -name '*.md'").read()?;
    let docs = docs.split('\n');
    cmd!(sh, "cp -r {docs...} {src_dir}/src/descriptions/").run()?;
    Ok(())
}
