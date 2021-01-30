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

use std::env;

#[derive(Debug)]
pub enum FileType {
	RawNFL,
	SplitNFL,
}

/// Returns the filename
pub fn read() -> Result<(String, FileType), String> {
	// Argument variables
	let mut infile: String = String::from("");
    let mut in_file_type: Option<FileType> = None;

	// Get arguments
	{
		let args: Vec<String> = env::args().collect();
		println!("{:?}", args);

		let mut i: usize = 1;
		while i < args.len() {
            // TODO Handle other argument types
			infile = args[i].clone();
            
            // Proceed to next arg
            i += 1;
		}
	}
    
    if let None = in_file_type {
        if infile.ends_with(".nfl") {
        	if infile.ends_with("-split.nfl") {
				in_file_type = Some(FileType::SplitNFL);
            }
            else {
                in_file_type = Some(FileType::RawNFL);
            }
        }
        else {
            return Err(String::from("Can only process nfl files"));
        }
    }
    
    Result::Ok((infile, in_file_type.unwrap()))
}