use netconfig_rs::Interface;
use std::error::Error;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Index(IndexArgs),
    Name(NameArgs),
}
#[derive(Args)]
struct IndexArgs {
    index: u32,
}
#[derive(Args)]
struct NameArgs {
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let interface = match &cli.command {
        Commands::Index(index) => Interface::try_from_index(index.index)?,
        Commands::Name(name) => Interface::try_from_name(&name.name)?,
    };
    let mtu = interface.mtu()?;
    println!("current mtu: {mtu}");
    interface.set_mtu(1800)?;
    let mtu2 = interface.mtu()?;
    println!("after: {mtu2}");
    //interface.set_mtu(mtu)?;
    Ok(())
}
