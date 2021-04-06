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

mod line;

use line::Line;
use std::collections::HashMap;
//use crate::assembly::line::Line;
//use self::line;

#[derive(Debug)]
pub struct Part {
	pub level: u64,
	pub data: HashMap<String, String>,
	pub lines: Vec<Line>,
	pub arcs: Vec<String>
}

impl Part {
	/// Creates a new part from the set of raw data from the file.
	/// This data does not need to be processed before hand, just each
	/// line should be separate, as it was in the file.
	pub fn new(level: u64, data: Vec<String>) -> Part {
		
		//let test_line = Line::new(String::from("\"L00017=LINE/0.7577722283114,-0.4375,0.7577722283114,0.4375\","));
		//println!("{:?}", test_line);
		
		let mut result = Part{
			level,
			data: HashMap::new(),
			lines: Vec::new(),
			arcs: Vec::new()
		};
		
		let line_tag = "LINE/";
		let circle_tag = "CIRCLE/";
		let line_escape = '$';
		let data_separator = '/';
		
		// We'll use a while loop because we sometimes need to consume
		// multiple lines in one loop. (Lines can be broken up with '$' chars.)
		let mut i: usize = 0;
		while i < data.len() {
			let mut line = data[i].clone();
			
			while line.ends_with(line_escape) {
				// Remove trailing '$'
				line = String::from(line.trim_end_matches(line_escape));

				i += 1;
				line.push_str(data[i].trim());
			}
			
			if line.contains(line_tag) {
				result.lines.push(Line::new(line));
			}
			else if line.contains(circle_tag) {
				result.arcs.push(line);
			}
			else {
				// Making hard assumption that this is formatted right
				let split = line.find(data_separator).unwrap();
				result.data.insert(
					String::from(&line[..split]), // Part before data separator
					String::from(&line[split+1..])// Part after
				);
			}
			
			i += 1;
		}
		
		result
	}
}
