use structopt::StructOpt;

use rainy::toolchain::Toolchain;
use rainy::update;

use log::info;

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
    env_logger::init();
    match Args::from_args() {
        Args::Update { toolchain } => info!("Specified toolchain: {}", toolchain),
    }
    if let Err(e) = update::update() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
