use anyhow::Result;
use home::home_dir;
use serde_json::from_slice;
use std::io::{Write, Read};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthdayDto {
	pub name: String,
	pub month: u32,
	pub day: u32,
}

impl From<Birthday> for BirthdayDto {
	
	fn from(birthday: Birthday) -> Self {
		Self { 
			name: birthday.name,
			month: birthday.month, 
			day: birthday.day,
		}
	}

}

pub fn save(birthdays: Vec<Birthday>) -> Result<()> {
	let mut file = File::create(get_file_path()?)?;
	let mut birthdays_dto = vec![];
	for birthday in birthdays {
		birthdays_dto.push(BirthdayDto::from(birthday));
	}
	let data = serde_json::to_string(&birthdays_dto)?;
	file.write_all(data.as_bytes())?;
	Ok(())
}

pub fn load() -> Result<Vec<Birthday>> {
	match File::open(get_file_path()?) {
		Ok(mut file) => {
			let mut data = vec![];
			let _ = file.read_to_end(&mut data);
			let birthdays_dto: Vec<BirthdayDto> = from_slice(&data)?;
			let mut birthdays = vec![];
			for birthday_dto in birthdays_dto {
				birthdays.push(Birthday::from(birthday_dto));
			} 
			Ok(birthdays)
		},
		Err(_) => {
			Ok(Vec::new())
		}
	}
}

pub fn nuke() -> Result<()> {
	Ok(fs::remove_file(get_file_path()?)?)
}