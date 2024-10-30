use clap::Parser;

mod cli;
mod auth;
mod name_of;
mod server_of;
mod last_message_id;
mod show_message;

#[tokio::main]
// There is no any return stamps,
// but clippy complains about needless return stamp
#[allow(clippy::needless_return)]
async fn main()  {
	let args = cli::Cli::parse();
	
	if args.command.is_none() {
		if let Err(error) = cli::app::main(
			args.selected_channel_id.expect("--channel is required")
		).await { println!("{error}") };
		return;
	}
	
	if args.selected_channel_id.is_some() {
		panic!("--channel is unsupported for subcommnads");
	}
	
	// Some(_) might be reachable 
	// once a new subcommand is introduced without a code
	#[allow(unreachable_patterns)]
	match args.command.unwrap() {
		cli::Commands::Unreads { with_id, mentions_only, server_id }
			=> cli::unreads::main(with_id, mentions_only, server_id).await,
		
		_ => unimplemented!()
	};
}
