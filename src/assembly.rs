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

//mod line;
pub mod part;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use part::Part;

#[derive(Debug)]
pub struct Assembly {
	pub header: Vec<String>,
	pub footer: Vec<String>,
	
	pub parts: Vec<Part>,
}

// Used when reading
enum FileSection { Header, Body, Footer }

impl Assembly {
	/// Creates a new assembly based on the given input file.
	/// 
	/// # Arguments
	/// 
	/// * infile: path to file
	pub fn new(infile: &str) -> Result<Assembly, &'static str> {

		// TODO test file missing
		
		let part_begin = "LEVEL/";
		let footer_begin = "FINI/";
		
		let mut result = Assembly {
			header: Vec::new(),
			footer: Vec::new(),
			parts: Vec::new()
		};
		
		if let Ok(lines) = read_lines(infile) {
			// Consumes the iterator, returns an (Optional) String
			let mut current_section = FileSection::Header;

			let mut part_data: HashMap<u64, Vec<String>> = HashMap::new();
			let mut level: u64 = 0;

			for line in lines {
				
				if let Ok(ip) = line {
					// This is so that Eclipse understands what this is
					let ip: String = ip;

					// This always denotes a new part, whether in the header
					// or in the body.
					if ip.starts_with(part_begin) {
						current_section = FileSection::Body;

						level = ip.strip_prefix(part_begin).unwrap().parse().unwrap_or(0);
						//result.parts.push(Part::new(level));
					}
					else {
						match current_section {
							FileSection::Header => result.header.push(ip),
							FileSection::Footer => result.footer.push(ip),
							FileSection::Body => {
								if ip.eq(footer_begin) {
									current_section = FileSection::Footer;
									result.footer.push(ip);
								}
								else {
									// TODO
									//result.parts.last_mut().unwrap().data.push(ip);
									//partData.get_mut(&level).unwrap().push(ip);
									part_data.entry(level).or_insert(Vec::new()).push(ip);
								}
							}
						}
					}
				}
			}
			
			for entry in part_data {
				result.parts.push(Part::new(entry.0, entry.1));
			}
		}
		
		Ok(result)
	}
	
	pub fn to_nfl(&self) -> String {
		let mut result: String = String::new();
		
		for l in &self.header {
			result.push_str(l);
			result.push('\n');
		}

		// TODO body

		for l in &self.footer {
			// Do this because NFL files have this weird symbol at the very end,
			// and we probably don't want to dump a newline afterward.
			result.push('\n');
			result.push_str(l);
		}

		
		return result;
	}
}

// Yoinked from
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}