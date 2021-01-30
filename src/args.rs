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
			if target.ends_with(".nfl") {
				if target.ends_with("-split.nfl") {
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
		
		Ok(Config{ target: target, target_type: target_type.unwrap()})
	}
}