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


mod args;


fn main() {
	match args::read() {
		Ok((infile, filetype)) => {
			println!("{} {:?}", infile, filetype);

			std::process::exit(0);
		}
		Err(msg) => {
			println!("{}", msg);
			std::process::exit(1);
        }
    }
}