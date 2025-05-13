use anyhow::Result;
use home::home_dir;
use serde_json::from_slice;
use std::io::{Write, Read};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use crate::birthday::Birthday;

pub fn get_main_dir() -> String {
	let mut path = home_dir().unwrap();
	path.push(".coolbirthday");
	path.to_str().unwrap().to_string()
}

pub fn init() -> Result<()>{
	let path = get_main_dir();
	if !Path::new(&path).exists() {
		fs::create_dir(path)?;
		Ok(())
	} else {
		Ok(())
	}
}

fn get_file_path() -> Result<PathBuf> {
	let mut path = PathBuf::from(&get_main_dir());
	path.push("coolbirthday_save");
	path.set_extension("json");
	Ok(path)
}

pub fn save(birthdays: Vec<Birthday>) -> Result<()> {
	let mut file = File::create(get_file_path()?)?;
	let data = serde_json::to_string(&birthdays)?;
	file.write_all(data.as_bytes())?;
	Ok(())
}

pub fn load() -> Result<Vec<Birthday>> {
	match File::open(get_file_path()?) {
		Ok(mut file) => {
			let mut data = vec![];
			let _ = file.read_to_end(&mut data);
			let birthdays: Vec<Birthday> = from_slice(&data)?;
			Ok(birthdays)
		},
		Err(_) => {
			Ok(Vec::new())
		}
	}
}