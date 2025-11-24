use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

use anyhow::{Context, Result, anyhow};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Build helper for gdiroh
struct Arguments {
    #[arg(short, long = "release", default_value = "false")]
    /// Builds gdiroh in the release profile
    release_build: bool,
    #[arg(
        short,
        long = "target",
        default_value = "../godot/gdiroh-example/addons/gdiroh"
    )]
    /// The target folder to move the compiled library to
    target_library_folder: PathBuf,
}

fn do_build(args: &Arguments) -> Result<()> {
    let mut build_command = Command::new("cargo");
    build_command.arg("build");
    if args.release_build {
        build_command.arg("--release");
    }

    let build_status = build_command.status().context("Failed to run cargo")?;

    if !build_status.success() {
        // We don't need an error message here, cargo should spit one out
        process::exit(1);
    }

    Ok(())
}

fn get_library_source(args: &Arguments) -> Result<(PathBuf, String)> {
    let xtask_root = &env::var("CARGO_MANIFEST_DIR")
        .context("The `CARGO_MANIFEST_DIR` variable does not exist, cargo may be broken")?;

    let Some(gdiroh_root) = Path::new(xtask_root).parent() else {
        return Err(anyhow!("The xtask crate has no parent directory"));
    };

    let target_dir = match args.release_build {
        false => "debug",
        true => "release",
    };

    // use env::consts to handle different OSes using different lib names
    let library_name = format!(
        "{}gdiroh{}",
        env::consts::DLL_PREFIX,
        env::consts::DLL_SUFFIX
    );

    Ok((
        gdiroh_root.join(format!("target/{}/{}", target_dir, library_name)),
        library_name,
    ))
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    do_build(&args)?;

    let (source_file, source_name) = get_library_source(&args)?;
    let target_file = args.target_library_folder.join(source_name);

    fs::copy(source_file, target_file).context("Copying the compiled library failed")?;

    Ok(())
}
