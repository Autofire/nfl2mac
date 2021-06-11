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
use std::fmt;
use std::ops::{Index, IndexMut};
//use log::*;
use euclid::{Point2D, Vector2D};

#[derive(Debug)]
pub struct Line {

	// "Lnnnnn=LINE/x1,y1,x2,y2",
    p1: Point2D<f64, f64>,
    p2: Point2D<f64, f64>
}

impl Line {
	
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Line {
        Line{ p1: Point2D::new(x1, y1), p2: Point2D::new(x2, y2) }
    }

	pub fn from_nfl(data: &str) -> Line {
		
		let trimmer = Regex::new(r"^.*/").unwrap();
		let data = String::from(trimmer.replace_all(data, ""));
		let split = data.split(',');
		let mut converted = split.map(|x| x.parse::<f64>().unwrap());
		
        Line::new(
            converted.next().unwrap(), converted.next().unwrap(),
            converted.next().unwrap(), converted.next().unwrap()
        )
	}

    /// If a and b overlap, gets the start and end points of the
    /// overlapping segment. If they do not overlap, None is returned.
    ///
    /// Note that, if the lines perfectly match, None is ALSO returned!
    pub fn find_overlaps(a: &Line, b: &Line, max_dist: f64)
        -> Option<(Point2D<f64, f64>, Point2D<f64, f64>)> 
    {
        let mut contained: Vec<Point2D<f64, f64>> = Vec::new();

        let mut append_if_contained = |l: &Line, p: &Point2D<f64,f64>| {
            if l.contains(p, max_dist) {
                contained.push(p.clone());
            }
        };

        // We'll figure out which points are contained in the lines.
        append_if_contained(&a, &b.p1);
        append_if_contained(&a, &b.p2);
        append_if_contained(&b, &a.p1);
        append_if_contained(&b, &a.p2);

        // If we see three overlaps, then the lines share at least one point.
        // If we see four, the lines perfectly match and nothing more must be done.
        if contained.len() == 2 {
            // Exactly two points are contained; they must be the endpoints
            Some((contained[0], contained[1]))
        }
        else if contained.len() == 3 {
            // Exactly one point is shared. We only need to do one check:
            // if the first two are VERY close, then we ignore one of 'em.
            // Otherwise, the third one is very close to either one of the
            // two. We don't care which one, we just can ignore it.

            if (contained[0]-contained[1]).square_length() <= max_dist * max_dist {
                Some((contained[1], contained[2]))
            }
            else {
                Some((contained[0], contained[1]))
            }
        }
        else {
            // Either lines perfectly match (four points) or something else is
            // arwy (lines are skew, etc.) Whatever the case, nothing needs to
            // be done here.
            None
        }
    }

    pub fn contains(&self, point: &Point2D<f64, f64>, max_dist: f64) -> bool {
        
        // TODO See about caching. We call this function a bunch of times!

        let v = self.to_vector();
        let l = v.length();

        // D = ||L x p0p1||/||L|| = ||a1a2 x a1b1||/||a1a2||
        let d = v.cross(self.p1 - *point) / l;
        if d.abs() > max_dist {
            return false;
        }
        else {
            // Alright, we've established that this point is on the (infinite)
            // line described by our endpoints.
            //
            // Now this distance from our desired point to BOTH endpoints must
            // be less than the distance between the endpoints themselves.
            // Only THEN can our point be on the line!
            return
                (*point - self.p1).square_length() < l*l &&
                (*point - self.p2).square_length() < l*l;
        }
        
    }


    /// Converts the line to a vector without the positional data.
    /// 
    /// This ensures that the y value is always positive. (i.e. if vector was
    /// placed at the origin, the end point would always be above the x axis.)
    pub fn to_vector(&self) -> Vector2D<f64, f64> {
        let diff = self.p1 - self.p2;

        if diff.y < 0.0 {
            diff * -1.0
        }
        else {
            diff
        }
    }

	pub fn to_nfl(&self, id: u64) -> String {
        let p1 = &self.p1;
        let p2 = &self.p2;

		format!("L{:0>5}=LINE/{},{},{},{}",
			id, p1.x, p1.y, p2.x, p2.y
		)
	}
	
}

impl Index<usize> for Line {
    type Output = Point2D<f64,f64>;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.p1,
            1 => &self.p2,
            _ => panic!("Index out of bounds: {}", i)
        }
    }
}

impl IndexMut<usize> for Line {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.p1,
            1 => &mut self.p2,
            _ => panic!("Index out of bounds: {}", i)
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p1 = &self.p1;
        let p2 = &self.p2;
        write!(f, "Line(({}, {}), ({}, {}))", p1.x, p1.y, p2.x, p2.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nfl() {
		let line = Line::from_nfl(String::from("L00017=LINE/1,1,2,2").as_str());

        assert_eq!(line.p1, Point2D::new(1.,1.));
        assert_eq!(line.p2, Point2D::new(2.,2.));

        assert_eq!(line.to_nfl(101), String::from("L00101=LINE/1,1,2,2"));
    }

    #[test]
    fn contains() {
		let l = Line::new(0., 0., 2., 2.);

        assert!(l.contains(&Point2D::new(0., 0.), 0.00001));
        assert!(l.contains(&Point2D::new(1., 1.), 0.00001));
        assert!(l.contains(&Point2D::new(2., 2.), 0.00001));

        assert!(!l.contains(&Point2D::new(0., 2.), 0.00001));
        assert!(!l.contains(&Point2D::new(2., 0.), 0.00001));
        assert!(l.contains(&Point2D::new(0., 2.), 1.5));

    }
}
