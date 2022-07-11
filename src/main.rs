mod syscallapi;
use syscallapi::api;
use std::env;


fn main() {
	let client = api::SyscallApiClient::new();
	let args: Vec<String> = env::args().collect();

	let syscall_name = &args[1];
	let arch = &args[2];

	match client.get_arch_syscall(arch.to_string(), syscall_name.to_string()) {
		Some((syscall, convention)) => {
			println!("Syscall {}\n{}", syscall_name, syscall.args_to_string(convention));
		},
		None => println!("Unable to find read for x64")
	}

}
