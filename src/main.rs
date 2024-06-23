mod fs_server;

use fs_server::FsServer;
use clap::{arg, command, Command};

#[tokio::main]
async fn main() {
    let command_result = command!()
		.subcommand(
			Command::new("serve")
				.about("Starts filesystem server.")
				.arg(
					arg!(-v --verbose "Enables debug/verbose mode.")
				)
		)
		.get_matches();

	match command_result.subcommand() {
		Some(("serve", _argument_matches)) => {
			let mut fs_server = FsServer::new();

			let watch_handle = fs_server.watch().await;
			let server_handle = fs_server.start().await;
			
			let (_, _) = tokio::join!(watch_handle, server_handle);
		},

		_ => ()
	}
}
