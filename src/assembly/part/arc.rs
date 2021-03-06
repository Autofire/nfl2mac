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

use regex::Regex;
use euclid::Point2D;

const DEFAULT_GO_ANGLE: f64 = 0.0;
const DEFAULT_END_ANGLE: f64 = 360.0;

#[derive(Debug)]
pub struct Arc {

	// "C00023=CIRCLE/CENTER,0.8045921410692,1.9847924952923,RADIUS,0.4838867345605,GOANG,90.140087201328,ENDANG,256.45889291795"
    /*
	pub x: f64,
	pub y: f64,
    */
    pub center: Point2D<f64, f64>,
	pub radius: f64,
	pub go_angle: f64,
	pub end_angle: f64
}

impl Arc {

	pub fn new(data: &str) -> Arc {
		let mut result = Arc{
			center: Point2D::new(0.0, 0.0), radius: 0.0,
			go_angle: DEFAULT_GO_ANGLE, end_angle: DEFAULT_END_ANGLE
		};

		let trimmer = Regex::new(r"^.*/").unwrap();
		let data = String::from(trimmer.replace_all(data, ""));
		let mut split = data.split(',');
		
		let mut next = split.next();
		while next != None {
			let s = next.unwrap();
			
			if s == "CENTER" {
				result.center.x = split.next().unwrap().parse::<f64>().unwrap();
				result.center.y = split.next().unwrap().parse::<f64>().unwrap();
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

	pub fn to_nfl(&self, id: u64) -> String {

        let delim = ",";
        let line_break = "$\n";
        let line_break_post = "              ";
        let max_line_len = 79;

        // All lines have this, and they'll probably be on the same line.
		let mut result = format!("C{:0>5}=CIRCLE/CENTER,{},{},RADIUS,{}", id, self.center.x, self.center.y, self.radius);

        let mut parts = Vec::new();
		if self.go_angle  != DEFAULT_GO_ANGLE  { parts.push(format!("GOANG,{}", self.go_angle)); }
		if self.end_angle != DEFAULT_END_ANGLE { parts.push(format!("ENDANG,{}", self.end_angle)); }

        if parts.len() > 0 {
            let angle_part = parts.join(delim);

            result.push_str(delim);
            if result.len() + angle_part.len() + line_break.len() + delim.len() >= max_line_len {
                result.push_str(line_break);
                result.push_str(line_break_post);
            }

            result.push_str(&angle_part);
        }
		
		result
	}
}

