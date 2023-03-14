use std::collections::HashMap;
use std::iter::zip;
use crate::weibull::{WeibullModel, reliability};
use serde::{Serialize, Deserialize};
use chrono::{Duration, NaiveDate};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeSeries {
    pub data: Vec<TimeDatum>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeDatum {
    pub time: NaiveDate,
    pub data: f64,
}

impl TimeDatum {
    pub fn new(time: NaiveDate, data: f64) -> Self {
        Self { time, data }
    }
}

impl TimeSeries {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate, weibull: WeibullModel) -> Self {
        let date_range = create_date_range(start_date, end_date);
        let num_steps = date_range.len();
        let reliability = reliability(weibull.weibull, num_steps as u32);
        let mut series_data = zip(date_range, reliability);

        let mut series: Vec<TimeDatum>= Vec::new();
        for (time, data) in series_data {
            let time_datum = TimeDatum::new(time, data);
            series.push(time_datum);
        }

        Self { data: series }
    }
}

/// Create a series of dates, in days
fn create_date_range(start_date: NaiveDate, end_date: NaiveDate) -> Vec<NaiveDate> {
    (0..=end_date.signed_duration_since(start_date).num_days())
        .map(|i| start_date + Duration::days(i))
        .collect()
}

/// Create a map from NaiveDate to index
///
/// This is useful when indexing into our time series. Given that 
/// a `TimeSeries` is built from a `Vec<NaiveDate`s, 
fn date_index_map(dates: Vec<NaiveDate>) -> HashMap<NaiveDate, usize> {
    dates
        .iter()
        .enumerate()
        .map(|(index, &date)| (date, index))
        .collect()
}
