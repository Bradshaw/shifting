use fs_extra::dir::CopyOptions;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    let channel_arg = env::args().nth(2);
    let mut channel: &str = "debug";
    match channel_arg.as_ref().map(|it| it.as_str()) {
        Some("debug") => channel = "debug",
        None => channel = "debug",
        Some("release") => channel = "release",
        _ => print_help(),
    }
    match task.as_ref().map(|it| it.as_str()) {
        Some("build") => build(channel)?,
        Some("dist") => dist(channel)?,
        Some("run") => run(channel)?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

dist            builds application and man pages
"
    )
}

fn build(channel: &str) -> Result<(), DynError> {
    build_binary(channel)?;
    Ok(())
}

fn dist(channel: &str) -> Result<(), DynError> {
    dist_files(channel)?;
    Ok(())
}

fn run(channel: &str) -> Result<(), DynError> {
    build(channel)?;
    dist(channel)?;

    let status = Command::new(format!("./dist/{channel}/shifting")).status()?;

    if !status.success() {
        Err("launching game failed")?;
    }
    Ok(())
}

fn build_binary(channel: &str) -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);

    command.current_dir(project_root());

    if channel == "release" {
        command.args(&["build", "--release"]);
    } else {
        command.args(&["build"]);
    }

    let status = command.status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    Ok(())
}

fn dist_files(channel: &str) -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir(channel));
    fs::create_dir_all(&dist_dir(channel))?;

    let dst = project_root().join(format!("target/{channel}/shifting").as_str());

    fs::copy(&dst, dist_dir(channel).join("shifting"))?;
    fs_extra::dir::copy(
        project_root().join("resources"),
        dist_dir(channel).join("resources"),
        &CopyOptions {
            overwrite: true,
            skip_exist: true,
            buffer_size: 64000,
            copy_inside: true,
            content_only: false,
            depth: 0,
        },
    )?;

    if Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        eprintln!("stripping the binary");
        let status = Command::new("strip").arg(&dst).status()?;
        if !status.success() {
            Err("strip failed")?;
        }
    } else {
        eprintln!("no `strip` utility found")
    }

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir(channel: &str) -> PathBuf {
    project_root().join(format!("dist/{channel}"))
}
