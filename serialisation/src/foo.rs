use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataPoint {
    epoch: i64,
    name: String,
    details: String,
    value: u8,
    flag: bool,
}

#[derive(Debug, Default)]
pub struct DataPointBuilder {
    epoch: i64,
    name: String,
    details: String,
    value: u8,
    flag: bool,
}

impl DataPointBuilder {
    pub fn new() -> DataPointBuilder {
        DataPointBuilder::default()
    }

    pub fn epoch(mut self, epoch: i64) -> DataPointBuilder {
        self.epoch = epoch;
        self
    }

    pub fn name(mut self, name: &str) -> DataPointBuilder {
        self.name = name.to_owned();
        self
    }

    pub fn details(mut self, details: &str) -> DataPointBuilder {
        self.details = details.to_owned();
        self
    }

    pub fn value(mut self, value: u8) -> DataPointBuilder {
        self.value = value;
        self
    }

    pub fn flag(mut self, flag: bool) -> DataPointBuilder {
        self.flag = flag;
        self
    }

    pub fn build(self) -> DataPoint {
        DataPoint {
            epoch: self.epoch,
            name: self.name,
            details: self.details,
            value: self.value,
            flag: self.flag,
        }
    }
}
