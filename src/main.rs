use std::str::FromStr;

use structopt::StructOpt;

use failure::Error;

#[derive(Debug)]
enum Toolchain {
    Version(String),
    Master,
}

impl FromStr for Toolchain {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "master" => Ok(Toolchain::Master),
            _ => Ok(Toolchain::Version(s.to_string())),
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
    println!("{:?}", opt);
}
