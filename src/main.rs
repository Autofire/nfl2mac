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


mod args;
mod assembly;

use std::{env, process, fs, io};

use args::{Config, FileType};
use assembly::Assembly;

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args).unwrap_or_else(|err| {
		println!("{}", err);
		process::exit(1);
	});

	println!("{} {:?}", config.target, config.target_type);
	
	let mut asm = Assembly::new(&config.target).unwrap();
	//println!("{:#?}", asm);
    
	match config.target_type {
		FileType::RawNFL => {
			println!("Raw file... will split and store in {}", config.split_dest());	
			//println!("{}", asm.to_nfl());
            asm.split();
			fs::write(config.split_dest(), asm.to_nfl())?;
		},
		FileType::SplitNFL => println!("Already split... not splitting"),
	}
	
	return Ok(());
}
