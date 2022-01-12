use std::error::Error;

use crate::key;
use crate::pretty::*;

pub enum Collections {
    Patent,
    PatentIssued,
    Software,
    Spinoff,
}

#[derive(Debug, PartialEq)]
pub struct TechTransferClient {
    base_url: String,
}

impl TechTransferClient {
    pub fn new() -> Self {
        TechTransferClient {
            base_url: String::from("https://api.nasa.gov/techtransfer/patent/?"),
        }
    }

    /// Switches Collection
    pub fn switch(&mut self, collection: Collections) -> Result<(), Box<dyn Error>> {
        match collection {
            Collections::Patent => {
                self.base_url = String::from("https://api.nasa.gov/techtransfer/patent/?");
                Ok(())
            }
            Collections::PatentIssued => {
                self.base_url =
                    String::from("https://api.nasa.gov/techtransfer/patent_issued/?");
                Ok(())
            }
            Collections::Software => {
                self.base_url = String::from("https://api.nasa.gov/techtransfer/software/?");
                Ok(())
            }
            Collections::Spinoff => {
                self.base_url = String::from("https://api.nasa.gov/techtransfer/spinoff/?");
                Ok(())
            }
        }
    }

    pub fn query(&self, query: String) -> Result<String, Box<dyn Error>> {
        let key: String = key::from_dotenv()?;

        let url = format!("{}{}&api_key={}", self.base_url, query, key);

        let res: String = ureq::get(&url).call()?.into_string()?;
        let tech = to_string_pretty(res);

        if tech.is_ok() {
            Ok(tech.unwrap())
        } else {
            Err(tech.unwrap_err())
        }
    }
}