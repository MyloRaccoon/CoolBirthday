use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "coolbirthday", about="Cool Birthday -- never forget one again !")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	#[command(about="display the version of Cool Birthday")]
	Version,

	#[command(about="list all registered birthdays")]
	List,

	#[command(about="print the next upcoming birthday")]
	Check {
		#[arg(help="(optional) filter by a specific name")]
		name: Option<String>,
	},

	#[command(about="add a new birthday")]
	Add {
		#[arg(help="name of the person")]
		name: String,

		#[arg(help="month of birth")]
		month: u32,

		#[arg(help="day of birth")]
		day: u32,
	},

	#[command(about="remove a birthday by name")]
	Remove {
		#[arg(help="name of the person")]
		name: String,
	},

	#[command(about="remove every registered birthdays")]
	Nuke,
}