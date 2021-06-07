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

use assembly::Assembly;
use args::{Config, FileType};

use std::{fs, error::Error, cmp::min};
use log::{info, debug};
use flexi_logger::Logger;


fn main() -> Result<(), Box<dyn Error>> {

    //let mut opt = Opt::from_args();
    let conf = Config::new()?;


    let log_levels = ["error", "warn", "info", "debug", "trace"];
    let log_level = log_levels[min(conf.verbose, log_levels.len()-1)];

    if !conf.quiet {
        Logger::with_env_or_str(log_level)
            .set_palette(String::from("196;208;-;7;108"))
            .start()
            .unwrap();
    }

    debug!("{:?}", conf);
    
    let mut asm = Assembly::new(&conf.target).unwrap();
    debug!("File contents:\n{}", asm.to_nfl());

    match conf.target_type.clone().expect("Failed to deduce input type") {
        FileType::RawNFL => {
            info!("Raw file... will split and store in {:?}", conf.split_dest());	
            asm.split();
            debug!("After split:\n{}", asm.to_nfl());
            fs::write(conf.split_dest(), asm.to_nfl())?;
        },
        
        FileType::SplitNFL => {
            println!("Already split... not splitting");
        },
    }
    
    Ok(())
}
