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
	pub contents: Vec<String>
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
					//let ip: String = ip;

					// This always denotes a new part, whether in the header
					// or in the body.
					if ip.starts_with("LEVEL/") {
						current_part = FilePart::Body;

						// TODO
						result.parts.push(Part{contents: vec![ip]});
					}
					else {
						match current_part {
							FilePart::Header => result.header.push(ip),
							FilePart::Footer => result.footer.push(ip),
							FilePart::Body => {
								if ip.eq("FINI/") {
									current_part = FilePart::Footer;
									result.footer.push(ip);
								}
								else {
									// TODO
									result.parts.last_mut().unwrap().contents.push(ip);
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