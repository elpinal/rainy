use std::fmt;
use std::str::FromStr;

use structopt::StructOpt;

use failure::{Error, Fail};

#[derive(Debug)]
enum Toolchain {
    Version {
        major: usize,
        minor: usize,
        patch: usize,
    },
    Master,
}

impl fmt::Display for Toolchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Toolchain::*;
        match *self {
            Version {
                major,
                minor,
                patch,
            } => write!(f, "{}.{}.{}", major, minor, patch),
            Master => write!(f, "master"),
        }
    }
}

/// A command error.
#[derive(Fail, Debug)]
pub enum CommandError {
    /// Invalid toolchain.
    #[fail(display = "invalid toolchain: {}", s)]
    InvalidToolchain { s: String },
}

fn parse_version(s: &str) -> Result<Toolchain, Error> {
    match &s.split('.').collect::<Vec<&str>>()[..] {
        [x, y, z] => Ok(Toolchain::Version {
            major: x.parse()?,
            minor: y.parse()?,
            patch: z.parse()?,
        }),
        _ => Err(Error::from(CommandError::InvalidToolchain {
            s: s.to_string(),
        })),
    }
}

impl FromStr for Toolchain {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "master" => Ok(Toolchain::Master),
            _ => parse_version(s),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rainy", author = "", version_short = "v")]
/// Rainy.
enum Args {
    /// Update a toolchain.
    #[structopt(name = "update", author = "")]
    Update {
        /// Target toolchain.
        toolchain: Toolchain,
    },
}

fn main() {
    let opt = Args::from_args();
    match opt {
        Args::Update { toolchain } => println!("{}", toolchain),
    }
}
