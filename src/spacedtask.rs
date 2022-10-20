use crate::util::*;
use chrono::prelude::*;

lazy_static! {
    static ref MODIFIER: f32 = std::env::var("SPACED_INTERVAL_MODIFIER")
        .map(|val| val.parse::<f32>().unwrap_or(2.5))
        .unwrap_or(2.5);
    static ref MAX_INTERVAL: usize = std::env::var("SPACED_MAX_INTERVAL")
        .map(|val| val.parse::<usize>().unwrap_or(365 * 2))
        .unwrap_or(365 * 2);
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpacedTask {
    pub name: String,
    pub date: Option<chrono::NaiveDate>,
    pub interval: usize,
}

impl SpacedTask {
    pub fn new(title: String, date: Option<NaiveDate>) -> SpacedTask {
        let added = SpacedTask {
            name: title,
            date: Some(date.unwrap_or_else(|| Utc::now().date_naive())),
            interval: 1,
        };

        println!("Created: {added}");
        added
    }

    pub fn new_unscheduled(title: String) -> SpacedTask {
        let added = SpacedTask {
            name: title,
            date: None,
            interval: 1,
        };

        println!("Created: {added}");
        added
    }

    pub fn increase_interval(&mut self, custom_intervals: &[usize]) {
        match self.date {
            Some(date) => {
                self.interval = if !custom_intervals.is_empty() {
                    next_larger_interval(self.interval, custom_intervals)
                } else {
                    ((self.interval as f32 * *MODIFIER).ceil() as usize).min(*MAX_INTERVAL)
                };
                self.date
                    .map(|val| val + chrono::Duration::days(self.interval as i64));
            }
            None => {
                self.date = Some(Utc::now().date_naive());
                self.interval = 1;
            }
        }
        println!("Updated: {self}")
    }

    pub fn repeat_interval(&mut self) {
        self.date
            .map(|val| val + chrono::Duration::days(self.interval as i64));
        println!("Repeated: {self}")
    }

    pub fn reduce_interval(&mut self, custom_intervals: &[usize]) {
        if !custom_intervals.is_empty() {
            self.interval = next_smaller_interval(self.interval, custom_intervals);
        } else {
            self.interval = ((self.interval as f32 / *MODIFIER).ceil() as usize).min(*MAX_INTERVAL);
        }
        self.date
            .map(|val| val + chrono::Duration::days(self.interval as i64));
        println!("Hard updated {self}")
    }

    pub fn reset(&mut self, custom_intervals: &[usize]) {
        self.interval = *custom_intervals.first().unwrap_or(&1);
        self.date
            .map(|val| val + chrono::Duration::days(self.interval as i64));
        println!("Reset {self}")
    }
}

impl std::fmt::Display for SpacedTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}#{}",
            self.name,
            self.date.map(|val| format!("@{val} ")).unwrap_or_default(),
            self.interval
        )
    }
}
