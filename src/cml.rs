use crate::time::TimeSeries;
use crate::weibull::WeibullModel;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cml {
    pub id: u32,
    pub series: TimeSeries,
}

impl Cml {
    pub fn new(id: u32, start_date: NaiveDate, end_date: NaiveDate) -> Self {
        let weibull = WeibullModel::new(0.5, 200.0);
        let series = TimeSeries::new(start_date, end_date, weibull);
        Self { id, series }
    }
}
