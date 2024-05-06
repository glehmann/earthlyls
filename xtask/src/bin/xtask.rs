use std::io::Write;

use clap::{Args, Parser};
use toml_edit::{value, DocumentMut};
use xshell::{cmd, Shell};

/// Utility commands
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
#[command(name = "cargo xtask")]
#[command(bin_name = "cargo xtask")]
enum Command {
    /// Generate the earthly command description files
    GenerateDescriptions,
    /// Bump the version and create a new draft release on github
    Release(Release),
}

#[derive(Args, Debug)]
struct Release {
    /// The new version number
    version: String,
}

fn main() -> anyhow::Result<()> {
    let args = Command::parse();
    match args {
        Command::GenerateDescriptions => generate_descriptions()?,
        Command::Release(args) => release(&args)?,
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

fn release(args: &Release) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    // update the workspace Cargo.toml
    let toml = std::fs::read_to_string("Cargo.toml")?;
    let mut doc = toml.parse::<DocumentMut>()?;
    doc["workspace"]["package"]["version"] = value(&args.version);
    std::fs::File::create("Cargo.toml")?.write_all(doc.to_string().as_bytes())?;
    cmd!(sh, "cargo test").run()?;
    // update vscode extension’s package.json
    sh.change_dir("editor/vscode");
    let version = &args.version;
    cmd!(sh, "npm version {version}").run()?;
    // commit, tag and push
    let message = format!("bump version to {version}");
    cmd!(sh, "git commit -am {message}").run()?;
    cmd!(sh, "git tag -am {version} {version}").run()?;
    cmd!(sh, "git push").run()?;
    cmd!(sh, "git push --tags").run()?;
    eprintln!(
        "now wait for the release worflow to complete and publish the release on \
        https://github.com/glehmann/earthlyls/releases/tag/{version}"
    );
    Ok(())
}
