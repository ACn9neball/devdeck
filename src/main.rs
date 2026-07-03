mod database;
mod display;
mod input;
mod list;
mod readme;
mod search;

use clap::{Parser, Subcommand};
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about= None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(visible_alias = "-a")]
    Add,
    #[command(visible_alias = "-d")]
    Display { id: i64 },
    #[command(visible_alias = "-l")]
    List,
    #[command(visible_alias = "-e")]
    Edit { id: i64 },
    #[command(visible_alias = "-s")]
    Search { name: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "devdeck";
    let mut path = dirs::config_dir().ok_or("No system config file")?;
    path.push(name);
    fs::create_dir_all(&path)?;

    let data = "markdown.db";
    path.push(data);
    let database_path: &str = path.to_str().unwrap();
    database::db_setup(database_path).expect("!Create");
    let args = Args::parse();
    match &args.command {
        Commands::Add => input::start(0, 0).expect(""),
        Commands::Display { id } => display::view(id.clone()).expect(""),
        Commands::List => list::display().expect(""),
        Commands::Edit { id } => input::start(1, id.clone()).expect(""),
        Commands::Search { name } => search::display(name.clone()).expect(""),
    }
    Ok(())
}
