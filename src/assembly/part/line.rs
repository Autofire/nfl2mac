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
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Index, IndexMut};
use log::*;
use euclid::{Point2D, Vector2D};

#[derive(Debug)]
pub struct Line {

	// "Lnnnnn=LINE/x1,y1,x2,y2",
    //
    // These are RefCells because lines can share points.
    // Nesting it like this allows us to modify the points
    // and then have thise changes propogate to other lines.
    p1: Rc<RefCell<Point2D<f64, f64>>>,
    p2: Rc<RefCell<Point2D<f64, f64>>>
}

impl Line {
	
	pub fn new(data: &str) -> Line {
		
		let trimmer = Regex::new(r"^.*/").unwrap();
		let data = String::from(trimmer.replace_all(data, ""));
		let split = data.split(',');
		let mut converted = split.map(|x| x.parse::<f64>().unwrap());
		
        Line{
            p1: Rc::new(RefCell::new(Point2D::new(converted.next().unwrap(), converted.next().unwrap()))),
            p2: Rc::new(RefCell::new(Point2D::new(converted.next().unwrap(), converted.next().unwrap())))
        }
	}

    pub fn merge_points(a: &mut Line, b: &mut Line, max_dist: f64) {

        trace!("Attempting merge of {} and {}", a, b);

        for i in 0..1 {
            for j in 0..1 {
                trace!("Checking {:?} and {:?}", a[i], b[i]);
                if a[i].borrow().distance_to(*b[j].borrow()) <= max_dist {
                    debug!("Merging {:?} and {:?}", a[i], b[i]);
                    a[i] = b[j].clone();
                }
            }
        }
    }

    /// If a and b overlap, gets the start and end points of the
    /// overlapping segment. If they do not overlap, None is returned.
    pub fn find_overlaps(a: &Line, b: &Line, max_dist: f64)
        -> Option<(Point2D<f64, f64>, Point2D<f64, f64>)> 
    {
        /*
        // Rather than compute slopes, we're just going to check the distance
        // of the endpoints to the lines.
        let a_vec = a.to_vector();
        let a_len = a_vec.length();

        // D = ||L x p0p1||/||L|| = ||a1a2 x a1b1||/||a1a2||
        let d1 = a_vec.cross(a.p1 - b.p1) / a_len;
        if d1 > max_dist {
            return None;
        }

        let d2 = a_vec.cross(a.p1 - b.p2) / a_len;
        if d2 > max_dist {
            return None;
        }

        // If we've gotten this far, we know that b's endpoints are quite
        // close to the line described by a's endpoints.
        //
        // This means one of these things are possible:
        //  1.) One of b's points are contained in a
        //  2.) b is contained in a
        //  3.) a is contained in b
        //  4.) There is no overlap
        //
        // Since we've established that b's points are close to the line,
        // we'll check (square) distances. We know the point is inside segment
        // a if the distance between it and each of a's endpoints is under the
        // distance between the endpoints themselves.

        let a_len_sqr = a_len * a_len;
        // TODO create lambda
        let _b1_inside =
            (b.p1 - a.p1).square_length() < a_len_sqr &&
            (b.p1 - a.p2).square_length() < a_len_sqr;

        let _b2_inside =
            (b.p2 - a.p1).square_length() < a_len_sqr &&
            (b.p2 - a.p2).square_length() < a_len_sqr;
            */
        
        None
    }


    /// Converts the line to a vector without the positional data.
    /// 
    /// This ensures that the y value is always positive. (i.e. if vector was
    /// placed at the origin, the end point would always be above the x axis.)
    pub fn to_vector(&self) -> Vector2D<f64, f64> {
        let diff = *self.p1.borrow() - *self.p2.borrow();

        if diff.y < 0.0 {
            diff * -1.0
        }
        else {
            diff
        }
    }

	pub fn to_nfl(&self, id: u64) -> String {
        let p1 = *(self.p1).borrow();
        let p2 = *(self.p2).borrow();

		format!("L{:0>5}=LINE/{},{},{},{}",
			id, p1.x, p1.y, p2.x, p2.y
		)
	}
	
}

impl Index<usize> for Line {
    type Output = Rc<RefCell<Point2D<f64,f64>>>;

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
        let p1 = *(self.p1).borrow();
        let p2 = *(self.p2).borrow();
        write!(f, "<({}, {}), ({}, {})>", p1.x, p1.y, p2.x, p2.y)
    }
}
