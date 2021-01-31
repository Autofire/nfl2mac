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
mod assembly;

use std::env;
use std::process;

use args::Config;
use assembly::Assembly;

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args).unwrap_or_else(|err| {
		println!("{}", err);
		process::exit(1);
	});
	
	println!("{} {:?}", config.target, config.target_type);
	
	let asm = Assembly::new(&config.target).unwrap();
	println!("{:#?}", asm);
}