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

use regex::Regex;

#[derive(Debug)]
pub struct Line {

	// "Lnnnnn=LINE/x1,y1,x2,y2",
	pub x1: f64,
	pub y1: f64,
	pub x2: f64,
	pub y2: f64
}

impl Line {
	
	pub fn new(data: &str) -> Line {
		
		let trimmer = Regex::new(r"^.*/").unwrap();
		let data = String::from(trimmer.replace_all(data, ""));
		let split = data.split(',');
		let mut converted = split.map(|x| x.parse::<f64>().unwrap());
		
		Line{
			x1: converted.next().unwrap(),
			y1: converted.next().unwrap(),
			x2: converted.next().unwrap(),
			y2: converted.next().unwrap()
		}
		
		
	}
	
}