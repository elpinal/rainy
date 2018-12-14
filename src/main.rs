use structopt::StructOpt;

use rainy::toolchain::Toolchain;

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
    match Args::from_args() {
        Args::Update { toolchain } => println!("{}", toolchain),
    }
}
