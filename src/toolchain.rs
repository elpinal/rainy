use std::fmt;
use std::str::FromStr;

use failure::{Error, Fail};

#[derive(Debug)]
pub enum Toolchain {
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

/// A parse error.
#[derive(Fail, Debug)]
pub enum ParseError {
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
        _ => Err(Error::from(ParseError::InvalidToolchain {
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
