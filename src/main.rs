use clap::Parser;
mod syscallapi;
use syscallapi::api;

#[derive(Parser, Debug)]
#[clap(version = "1.0")]
#[clap(author, about, long_about = None)]
struct Args {
	/// Arch type to be searched
	#[clap(short, long, value_parser)]
	arch: String,

	/// Syscall name to be searched
	#[clap(short, long, value_parser)]
	syscall: String
}

fn main() {
	let args = Args::parse();

	let client = api::SyscallApiClient::new();

	let syscall_name = args.syscall;
	// let bits = 

	match client.get_arch_syscall(args.arch.to_string(),syscall_name.to_string()) {
		Some((syscall, convention)) => {
			println!("{}:\n{}", syscall_name, syscall.args_to_string(convention));
		},
		None => println!("Unable to find {} for {}", syscall_name, args.arch)
	}



}
