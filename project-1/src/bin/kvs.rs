use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"), 
    about = env!("CARGO_PKG_DESCRIPTION"), 
    author = env!("CARGO_PKG_AUTHORS"), 
    version=env!("CARGO_PKG_VERSION")
)]
enum Cli {
    /// Set the value of the given key
    Set {
        #[structopt(name = "KEY", required = true)]
        key: String,
        #[structopt(name = "VALUE", required = true)]
        value: String,
    },
    /// Get the value of the given key
    Get {
        #[structopt(name = "KEY", required = true)]
        key: String,
    },
    /// Remove the given key
    Rm {
        #[structopt(name = "KEY", required = true)]
        key: String,
    },
}

fn main() {
    match Cli::from_args() {
        Cli::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Cli::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Cli::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
