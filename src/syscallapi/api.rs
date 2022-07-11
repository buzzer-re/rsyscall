use super::types;

const SYSCALL_API_URL: &str = "https://api.syscall.sh/v1/syscalls/";
const SYSCALL_CALLE_CONVENTIONS_URL: &str = "https://api.syscall.sh/v1/conventions/";

pub struct SyscallApiClient {}

impl SyscallApiClient {
    pub fn new() -> SyscallApiClient {
        SyscallApiClient {}
    }

    pub fn get_arch_syscall(&self, arch_name: String, syscall_name: String) -> Option<(types::Syscall, types::Convention)> {
        
        let convention_url = format!("{}{}", SYSCALL_CALLE_CONVENTIONS_URL, arch_name);
        let syscall_url = format!("{}{}", SYSCALL_API_URL, arch_name);
		
		// TODO: Review errors output
		let convention = self.get(&convention_url).unwrap_or_else(|err| {
			panic!("Error on requesting calling convetion data for {}\nErr: {:?}", arch_name, err);
		});

		let syscalls = self.get(&syscall_url).unwrap_or_else(|err| {
			panic!("Error on requesting syscall list for {}\nErr: {:?}", syscall_url, err);
		});
			
		// Get calling convention specs
		let convention = types::Convention::from_str(convention.as_str()).unwrap_or_else(|err| {
			panic!("Error on parsing calling convetions data => {}", err);
		});


		let syscalls = types::Syscall::from_str_array(syscalls.as_str()).unwrap_or_else(|err| {
			panic!("Error on parsing syscall data => {}", err);
		});

		
		// Ok
		
		for syscall in syscalls.iter() {
			if syscall.name == syscall_name {
				return Some((syscall.clone(), convention));
			}
		}

		None

    }


    fn get(&self, url :&str) -> Result<String, reqwest::Error> {
		reqwest::blocking::get(url)?.text()
    }
}


