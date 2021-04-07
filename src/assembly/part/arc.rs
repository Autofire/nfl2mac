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
pub struct Arc {

	// "C00023=CIRCLE/CENTER,0.8045921410692,1.9847924952923,RADIUS,0.4838867345605,GOANG,90.140087201328,ENDANG,256.45889291795"
	pub x: f64,
	pub y: f64,
	pub radius: f64,
	pub go_angle: f64, // default 0
	pub end_angle: f64 // default 360
}

impl Arc {

	pub fn new(data: &str) -> Arc {
		let mut result = Arc{x: 0.0, y: 0.0, radius: 0.0, go_angle: 0.0, end_angle: 360.0};

		let trimmer = Regex::new(r"^.*/").unwrap();
		let data = String::from(trimmer.replace_all(data, ""));
		let mut split = data.split(',');
		
		let mut next = split.next();
		while next != None {
			let s = next.unwrap();
			
			if s == "CENTER" {
				result.x = split.next().unwrap().parse::<f64>().unwrap();
				result.y = split.next().unwrap().parse::<f64>().unwrap();
			}
			else if s == "RADIUS" {
				result.radius = split.next().unwrap().parse::<f64>().unwrap();
			}
			else if s == "GOANG" {
				result.go_angle = split.next().unwrap().parse::<f64>().unwrap();
			}
			else if s == "ENDANG" {
				result.end_angle = split.next().unwrap().parse::<f64>().unwrap();
			}
			
			next = split.next();
		}
		
		result
	}
}