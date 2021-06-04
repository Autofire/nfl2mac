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

pub mod line;
pub mod arc;

use line::Line;
use arc::Arc;
use std::collections::HashMap;
use euclid::Point2D;
//use crate::assembly::line::Line;
//use self::line;

#[derive(Debug)]
pub struct Part {
	pub level: u64,
	pub data: HashMap<String, String>,
	pub lines: Vec<Line>,
	pub arcs: Vec<Arc>
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
				result.lines.push(Line::new(line.as_str()));
			}
			else if line.contains(circle_tag) {
				result.arcs.push(Arc::new(line.as_str()));
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

    pub fn resolve_overlaps(a: &mut Part, b: &mut Part) {
        println!("Resolving parts {} and {}", a.level, b.level);

        // So the problem is that we need to replace lines... but we
        // cannot just add lines while we're looping through!
        // 
        // Instead, no splits will occur until we're done finding points
        // to split at. Once we do that, we'll go through this list of
        // points-to-split-at and perform them.
        // 
        // This way, if we need to perform multiple splits, it will be more
        // straightforward in the end.
        // 
        // When splits occur, we want to create points at average positions
        // between the overlapping areas. The new segments will share endpoints
        // between parts.
        // 
        // Note that we CANNOT destroy the endpoint!
        let _a_splits: HashMap<usize, Vec<Point2D<f64, f64>>> = HashMap::new();
        let _b_splits: HashMap<usize, Vec<Point2D<f64, f64>>> = HashMap::new();

        for i in 0..a.lines.len() {
            for j in 0..b.lines.len() {
                
                if let Some(overlaps)
                    = Line::find_overlaps(&a.lines[i], &b.lines[j], 0.01)
                {
                    println!("a:  {:?}", a);
                    println!("b:  {:?}", b);
                    println!("o0: {:?}", overlaps.0);
                    println!("o1: {:?}", overlaps.1);
                    println!();
                }
                
            }
        }
    }

	pub fn to_nfl(&self, id: &mut u64) -> String {
		let mut result = String::new();
		
		result += &format!("LEVEL/{}\n", self.level);
		
		for l in &self.data {
			result += &format!("{}/{}\n",l.0, l.1);
		}
		
		for l in &self.lines {
			result += &l.to_nfl(*id);
			result.push('\n');
			*id += 1;
		}

		for a in &self.arcs {
			result += &a.to_nfl(*id);
			result.push('\n');
			*id += 1;
		}
		
		result
	}
}
