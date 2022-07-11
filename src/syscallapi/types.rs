use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Result;



#[derive(Serialize, Deserialize)]
pub struct Convention {
	pub arch	:String,
	nr			:String,
	// ret     : String,
	arg0		:String,
	arg1		:String,
	arg2		:String,
	arg3		:String,
	arg4		:String,
	arg5		:String
}


// Same attributes but with different values

#[derive(Serialize, Deserialize, Clone)]
pub struct Syscall {
	pub arch	:String,
	pub name	:String,
	pub nr		:u32,
	// ret		:String,
	pub arg0	:String,
	pub arg1	:String,
	pub arg2	:String,
	pub arg3	:String,
	pub arg4	:String,
	pub arg5	:String
}

impl Convention {

	pub fn from_str(raw: &str) -> Result<Convention>  {
		serde_json::from_str(raw)
	}
}

impl Syscall {
	pub fn from_str_array(raw: &str) -> Result<Vec<Syscall>> {
		serde_json::from_str(raw)
	}

	pub fn args_to_string(&self, convention: Convention) -> String {
		// I don't like this :/
		// TODO: Improve this
		let mut output_fmt: String = String::from_str("").unwrap();
		output_fmt.push_str(format!("{} = {}\n", convention.arg0, self.arg0).as_str());
		output_fmt.push_str(format!("{} = {}\n", convention.arg1, self.arg1).as_str());
		output_fmt.push_str(format!("{} = {}\n", convention.arg2, self.arg2).as_str());
		output_fmt.push_str(format!("{} = {}\n", convention.arg3, self.arg3).as_str());
		output_fmt.push_str(format!("{} = {}\n", convention.arg4, self.arg4).as_str());
		output_fmt.push_str(format!("{} = {}\n", convention.arg5, self.arg5).as_str());

		output_fmt
	}
}


