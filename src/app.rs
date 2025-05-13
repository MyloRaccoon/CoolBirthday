use anyhow::Result;

use crate::{birthday::Birthday, file::{load, save}};

#[derive(Debug, Default)]
pub struct App {
	birthdays: Vec<Birthday>
}

impl App {

	pub fn new() -> Result<Self> {
		Ok(Self { birthdays: load()? })
	}

	pub fn save(&self) -> Result<()> {
		save(self.birthdays.clone())
	}

	pub fn load(&mut self) -> Result<()> {
		self.birthdays = load()?;
		Ok(())
	}

	pub fn get(&self, name: String) -> Option<Birthday> {
		self.birthdays.clone().into_iter().find(|birthday| birthday.name == name)
	}

	pub fn birthday_exists(&self, name: String) -> bool {
		self.get(name).is_some()
	}

	pub fn list(&self) {
		for birthday in self.birthdays.clone() {
			println!("{birthday}");
		}
	}

	pub fn check(&self, name: String) -> Option<bool> {
		self.get(name).map(|birthday| birthday.check())
	}

	pub fn check_all(&self) -> Option<Birthday> {
		self.birthdays.clone().into_iter().find(|birthday| birthday.check())
	}

	pub fn get_next(&self) -> Option<Birthday> {
		if self.birthdays.is_empty() {
			return None;
		}
		self.birthdays.iter().min_by_key(|birthday| birthday.get_next_date()).cloned()
	}

	pub fn add(&mut self, name: String, month: u32, day: u32) -> Result<Birthday> {
		let birthday = Birthday::new(name, month, day)?;
		self.birthdays.push(birthday.clone());
		Ok(birthday)
	}

	pub fn remove(&mut self, name: String) {
		self.birthdays.retain(|c_birthday| *c_birthday.name != name);
	}

	pub fn nuke(&mut self) {
		let names: Vec<String> = self.birthdays.clone().iter().map(|birthday| birthday.name.clone()).collect();
		for name in names {
			self.remove(name);
		}
	}

}

