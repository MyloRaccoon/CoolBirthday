use anyhow::Result;

use crate::{birthday::Birthday, file::{load, nuke, save}};

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

	pub fn list(&self) {
		for birthday in self.birthdays.clone() {
			println!("{birthday}");
		}
	}

	pub fn add(&mut self, name: String, month: u32, day: u32) -> Result<()> {
		self.birthdays.push(Birthday::new(name, month, day));
		self.save()
	}

	pub fn nuke() -> Result<()> {
		nuke()
	}

}

