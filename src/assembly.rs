// This file is part of nfl2mac.
// 
// nfl2mac is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// nfl2mac is distributed in the hope that it will be useful,
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
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use part::Part;

const SUB_CHAR: u8 = 26;	// This is what is read if EOF is not understood

#[derive(Debug)]
pub struct Assembly {
	pub header: Vec<String>,
	pub footer: Vec<String>,
	
	pub parts: Vec<Part>,
}

// Used when reading
enum FileSection { Header, Body(u64) , Footer }

impl Assembly {

	/// Creates a new assembly based on the given input file.
	/// 
	/// # Arguments
	/// 
	/// * infile: path to file
	pub fn new(infile: &PathBuf) -> Result<Assembly, &'static str> {

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

			// Was originally using HashMap, but it would mix up the order
			// of the parts, which meant our output changed with every run.
			// ...not great for diffs and stuff.
			let mut part_data: BTreeMap<u64, Vec<String>> = BTreeMap::new();

			for line in lines {
				
				if let Ok(ip) = line {
					// This is so that Eclipse understands what this is
					let ip: String = ip;

					// This always denotes a new part, whether in the header
					// or in the body.
					if ip.starts_with(part_begin) {
						current_section = FileSection::Body(
							ip.strip_prefix(part_begin)
								.unwrap()
								.parse()
								.unwrap_or(0)
						);
					}
					else if !ip.is_empty() && ip.as_bytes()[0] != SUB_CHAR {
						// On the line above, we check SUB_CHAR because Rust
						// sometimes reads that at the end of the file.

						match current_section {
							FileSection::Header => result.header.push(ip),
							FileSection::Footer => result.footer.push(ip),
							FileSection::Body(level) => {
								if ip.eq(footer_begin) {
									current_section = FileSection::Footer;
									result.footer.push(ip);
								}
								else {
									part_data.entry(level)
										.or_insert(Vec::new()).push(ip);
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
	
    pub fn split(&mut self) {
        for i in 0..self.parts.len() {
            for j in (i+1)..self.parts.len() {
                // This is a weird workaround... basically, forcing Rust
                // to allow us to pass in both indicies mutably.
                let (a, b) = self.parts.split_at_mut(j);

                Part::resolve_overlaps(&mut a[i], &mut b[0]);
            }
        }
        
    }

	/// Converts the assembly to an NFL string for writing.
	/// Contains newlines, does not end in newline.
	pub fn to_nfl(&self) -> String {
		let mut result: String = String::new();
		
		for l in &self.header {
			result += &l;
			result.push('\n');
		}

		// Individual parts start at ID 1
		let mut id = 1;
		for p in &self.parts {
			result += &p.to_nfl(&mut id);
		}


		for l in &self.footer {
			result += l;
			result.push('\n');
		}

		// Do this order because NFL files have this weird symbol at the
		// very end, and we probably don't want to dump a newline afterward.
		result.remove(result.len()-1);
		
		result
	}
}

// Yoinked from
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
