pub mod unreads;
pub mod app;

use clap::Subcommand;

#[derive(clap::Parser)]
#[command(about = "A CLI app for using Revolt API")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
	
	#[arg(
		long = "channel",
		short = 'C',
		help = "Messages from this channel will be shown"
	)]
	pub selected_channel_id: Option<String>
}

#[derive(Subcommand)]
pub enum Commands {
	#[command(about = "Check for unread messages")]
	Unreads {
		#[arg(
			long = "with-id", 
			help = "Show ID of messages and channels, can be used to refer to them in further commands"
		)]
		with_id: bool,
		
		#[arg(
			long = "mentions-only", 
			help = "Show only messages you was mentioned in"
		)]
		mentions_only: bool,
		
		#[arg(
			long = "server-id", 
			help = "ID of server to check unreads of"
		)]
		server_id: Option<String>,
	}
}
