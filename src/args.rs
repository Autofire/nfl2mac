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

// General structure yoinked from
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html

use structopt::StructOpt;
use std::path::PathBuf;

use strum_macros::EnumString;


const NFL_EXTENSION: &str = ".nfl";
const SPLIT_SUFFIX: &str = "-split";

#[derive(Debug)]
#[derive(EnumString)]
#[derive(Clone)]
pub enum FileType {
	RawNFL,
	SplitNFL,
}


#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "To be written.")]
pub struct Config {
    /// File to read
    #[structopt(parse(from_os_str), name = "FILE")]
	pub target: PathBuf,

    /// Makes output more verbose
    ///
    /// Adding extra v's (i.e. -vvv) increases verbosity, up to 4 times.
    /// Stages of verbosity are "error", "warn", "info", "debug", and "trace"
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,

    /// Suppresses output, overriding -v
    #[structopt(short, long)]
    pub quiet: bool,

    /// Type of the file being read
    ///
    /// If not given, will attempt to deduce it from <target>
    #[structopt(long)]
	pub target_type: Option<FileType>,

    /// Precision used when checking for overlaps in lines
    ///
    /// In the future, this may be auto-detected based on the part.
    /// Making this negative will cause lines to not be broken up.
    #[structopt(short, long, default_value = "0.000001")]
    pub line_precision: f64
}

impl Config {
	pub fn new() -> Result<Config, &'static str> {

        let mut conf = Config::from_args();

        // If we're missing the target type, we'll attempt to deduce it
        if conf.target_type.is_none() {

            if conf.target_name().ends_with(
                    format!("{}{}", SPLIT_SUFFIX, NFL_EXTENSION).as_str()
            ) {
                conf.target_type = Some(FileType::SplitNFL);
            }
            else {
                conf.target_type = Some(FileType::RawNFL);
            }

        }

        Ok(conf)

	}

    pub fn target_name(&self) -> &str {
        self.target
            .file_name().expect("Bad path")
            .to_str().expect("Bad path")
    }
	
	pub fn split_dest(&self) -> PathBuf {
		let mut new_name = String::from(self.target_name());
		new_name.insert_str(new_name.len() - NFL_EXTENSION.len(), SPLIT_SUFFIX);
		
		self.target.with_file_name(new_name)
	}
}
