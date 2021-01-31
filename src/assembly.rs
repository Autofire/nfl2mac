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

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct Part {
	pub level: u64,
	pub data: Vec<String>,
	pub lines: Vec<String>,
	pub arcs: Vec<String>
}

impl Part {
	pub fn new(level: u64) -> Part {
		Part{
			level,
			data: Vec::new(),
			lines: Vec::new(),
			arcs: Vec::new()
		}
	}
}

#[derive(Debug)]
pub struct Assembly {
	pub header: Vec<String>,
	pub footer: Vec<String>,
	
	pub parts: Vec<Part>,
}

// Used when reading
enum FilePart { Header, Body, Footer }

impl Assembly {
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
			let mut current_part = FilePart::Header;

			for line in lines {
				
				if let Ok(ip) = line {
					// This is so that Eclipse understands what this is
					let ip: String = ip;

					// This always denotes a new part, whether in the header
					// or in the body.
					if ip.starts_with(part_begin) {
						current_part = FilePart::Body;

						let level: u64 = ip.strip_prefix(part_begin).unwrap().parse().unwrap_or(0);
						result.parts.push(Part::new(level));
					}
					else {
						match current_part {
							FilePart::Header => result.header.push(ip),
							FilePart::Footer => result.footer.push(ip),
							FilePart::Body => {
								if ip.eq(footer_begin) {
									current_part = FilePart::Footer;
									result.footer.push(ip);
								}
								else {
									// TODO
									result.parts.last_mut().unwrap().data.push(ip);
								}
							}
							
						}
					}
				}
			}
		}
		
		Ok(result)
	}
}

// Yoinked from
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}