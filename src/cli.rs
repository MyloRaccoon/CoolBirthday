use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "coolbirthday")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	List,
	Add {
		name: String,
		month: u32,
		day: u32,
	},
	Nuke,
}