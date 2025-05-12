use chrono::{Datelike, NaiveDate};
use std::fmt;

use crate::file::BirthdayDto;

pub fn now() -> NaiveDate {
    chrono::Local::now().date_naive()
}

#[derive(Debug, Clone)]
pub struct Birthday {
    pub name: String,
    pub month: u32,
    pub day: u32
}

impl Birthday {

    pub fn new(name: String, month: u32, day: u32) -> Self {
        Self { 
            name, 
            month,
            day,
        }
    }

    pub fn get_next_date(&self) -> NaiveDate {
        let now = now();
        let date = NaiveDate::from_ymd_opt(now.year(), self.month, self.day).unwrap();
        if now > date {
            NaiveDate::from_ymd_opt(now.year() + 1, self.month, self.day).unwrap()
        } else {
            date
        }
    }

    pub fn get_days(&self) -> i64 {
        let now = now();
        let date = NaiveDate::from_ymd_opt(now.year(), self.month, self.day).unwrap();
        (date - now).num_days()
    }

    pub fn check(&self) -> bool {
        self.get_next_date() == now()
    }
}

impl From<BirthdayDto> for Birthday {
    
    fn from(birthday: BirthdayDto) -> Self {
        Self::new(birthday.name, birthday.month, birthday.day)
    }

}

impl fmt::Display for Birthday {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}/{}", self.name, self.day, self.month)
    }

}