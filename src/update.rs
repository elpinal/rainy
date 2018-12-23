use std::borrow::Cow;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use failure::{Error, Fail};

use log::trace;

const ROOT: &str = ".rain";
const BIN: &str = "bin";
const REPO: &str = "repo";
const RAIN_ML: &str = "rain-ml";
const RAIN_VM: &str = "rain-vm";

const RAIN_ML_URI: &str = "https://github.com/elpinal/rain-ml";
const RAIN_VM_URI: &str = "https://github.com/elpinal/rain-vm";

#[derive(Debug, Fail)]
enum UpdateError {
    #[fail(display = "command ({}) failed: {}", name, status)]
    ExternalCommand { name: String, status: ExitStatus },
}

fn root(home: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(home);
    path.push(ROOT);
    path
}

fn git() -> Command {
    Command::new("git")
}

fn stack() -> Command {
    Command::new("stack")
}

fn cargo() -> Command {
    Command::new("cargo")
}

fn clone(src: &str, dest: &Path) -> Result<(), Error> {
    let status = git().arg("clone").arg(src).arg(dest).status()?;
    if !status.success() {
        Err(UpdateError::ExternalCommand {
            name: "git".to_string(),
            status,
        })?
    }
    Ok(())
}

fn pull(dest: &Path) -> Result<(), Error> {
    let status = git().arg("pull").current_dir(dest).status()?;
    if !status.success() {
        Err(UpdateError::ExternalCommand {
            name: "git".to_string(),
            status,
        })?
    }
    Ok(())
}

fn update_or_clone(src: &str, dest: &Path) -> Result<(), Error> {
    trace!("Connecting: {}", src);
    if dest.exists() {
        trace!("Executing `git pull`.");
        pull(dest)?;
        trace!("Finished `git pull`.");
    } else {
        trace!("Executing `git clone`.");
        clone(src, dest)?;
        trace!("Finished `git clone`.");
    }
    Ok(())
}

fn stack_install<P, Q>(from: P, dest: Q) -> Result<(), Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path> + AsRef<std::ffi::OsStr>,
{
    let status = stack()
        .current_dir(from)
        .arg("--local-bin-path")
        .arg(dest)
        .arg("install")
        .status()?;
    if !status.success() {
        Err(UpdateError::ExternalCommand {
            name: "stack".to_string(),
            status,
        })?
    }
    Ok(())
}

fn cargo_install<P, Q>(from: P, dest: Q) -> Result<(), Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path> + AsRef<std::ffi::OsStr>,
{
    let status = cargo()
        .current_dir(from)
        .arg("install")
        .arg("--path")
        .arg(".")
        .arg("--force")
        .arg("--root")
        .arg(dest)
        .status()?;
    if !status.success() {
        Err(UpdateError::ExternalCommand {
            name: "cargo".to_string(),
            status,
        })?
    }
    Ok(())
}

fn create_root() -> Result<PathBuf, Error> {
    let home = env::var("HOME")?;
    let path = root(&home);
    if !path.exists() {
        trace!("Root does not exists.");
        trace!("Creating root directory.");
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

fn rain_ml_uri() -> Cow<'static, str> {
    env::var("RAINY_RAIN_ML_URI")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(RAIN_ML_URI))
}

fn rain_vm_uri() -> Cow<'static, str> {
    env::var("RAINY_RAIN_VM_URI")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(RAIN_VM_URI))
}

fn update_repo(ml: &Path, vm: &Path) -> Result<(), Error> {
    update_or_clone(&rain_ml_uri(), ml)?;
    update_or_clone(&rain_vm_uri(), vm)?;
    Ok(())
}

pub fn update() -> Result<(), Error> {
    trace!("Start updating.");
    let path = create_root()?;
    let ml = path.join(REPO).join(RAIN_ML);
    let vm = path.join(REPO).join(RAIN_VM);

    update_repo(&ml, &vm)?;

    stack_install(ml, path.join(BIN))?;
    cargo_install(vm, path)?;
    trace!("Finished updating.");
    Ok(())
}
