// This file is part of nfl2mac.
// 
// Foobar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// Foobar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with nfl2mac.  If not, see <https://www.gnu.org/licenses/>.

// General structure yoinked from
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html

const NFL_EXTENSION: &str = ".nfl";
const SPLIT_SUFFIX: &str = "-split";

#[derive(Debug)]
pub enum FileType {
	RawNFL,
	SplitNFL,
}

pub struct Config {
	pub target: String,
	pub target_type: FileType,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		// Argument variables
		let mut target: String = String::from("");
		let mut target_type: Option<FileType> = None;

		let mut i: usize = 1;
		while i < args.len() {
			// TODO Handle other argument types
			target = args[i].clone();
			
			// Proceed to next arg
			i += 1;
		}
		
		if target_type.is_none() {
			if target.ends_with(NFL_EXTENSION) {
				if target.ends_with(format!("{}{}", SPLIT_SUFFIX, NFL_EXTENSION).as_str()) {
					target_type = Some(FileType::SplitNFL);
				}
				else {
					target_type = Some(FileType::RawNFL);
				}
			}
			else {
				return Err("Can only process nfl files");
			}
		}
		
		let target_type = target_type.unwrap();
		Ok(Config{ target, target_type})
	}
	
	pub fn split_dest(&self) -> String {
		let mut result = String::from(&self.target);
		result.insert_str(result.len() - NFL_EXTENSION.len(), SPLIT_SUFFIX);
		
		result
	}
}