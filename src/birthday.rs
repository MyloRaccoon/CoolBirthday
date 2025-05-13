use anyhow::{Result, anyhow};
use chrono::{Datelike, NaiveDate};
use std::fmt;
use serde::{Serialize, Deserialize};

pub fn now() -> NaiveDate {
    chrono::Local::now().date_naive()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Birthday {
    pub name: String,
    pub month: u32,
    pub day: u32
}

impl Birthday {

    pub fn new(name: String, month: u32, day: u32) -> Result<Self> {
        if NaiveDate::from_ymd_opt(2000, month, day).is_none() {
            return Err(anyhow!("Coudn't create Birthday: date incorrect"));
        }
        Ok(Self{ name, month, day })
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
        let date = self.get_next_date();
        (date - now).num_days()
    }

    pub fn check(&self) -> bool {
        self.get_next_date() == now()
    }
}

impl fmt::Display for Birthday {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date = NaiveDate::from_ymd_opt(2000, self.month, self.day).unwrap();
        write!(f, "{}: {} (in {} days)", self.name, date.format("%d/%m"), self.get_days())
    }

}