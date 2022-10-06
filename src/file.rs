use anyhow::{anyhow, Error, Result};
use chrono::prelude::*;

const MODIFIER: f32 = 2.5;
const MAX_INTERVAL: usize = 365 * 2;

#[derive(Debug, PartialEq, Eq)]
pub struct SpacedTask {
    pub name: String,
    pub date: chrono::NaiveDate,
    pub interval: usize,
}

pub fn get_tasks(filename: &str) -> Result<Vec<SpacedTask>> {
    let mut tasks = Vec::new();
    let contents = std::fs::read_to_string(filename)?;
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        tasks.push(line.parse::<SpacedTask>()?);
    }
    Ok(tasks)
}

pub fn write_tasks(tasks: &[SpacedTask], filename: &str) -> Result<()> {
    std::fs::write(
        filename,
        tasks
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .map(|_| ())
    .map_err(|_| anyhow!("Failed to write tasks"))
}

impl std::str::FromStr for SpacedTask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').clone().collect();
        let name = parts[0];
        let dateparts = parts[1].split('-').collect::<Vec<&str>>();
        let date = NaiveDate::from_ymd(
            dateparts[0].parse()?,
            dateparts[1].parse()?,
            dateparts[2].parse()?,
        );
        let interval = parts[2].parse::<usize>()?;
        Ok(SpacedTask {
            name: name.to_string(),
            date,
            interval,
        })
    }
}

impl SpacedTask {
    pub fn new(title: String) -> SpacedTask {
        let added = SpacedTask {
            name: title,
            date: Utc::now().date_naive(),
            interval: 1,
        };

        println!("Created: {added}");
        added
    }

    pub fn update(&mut self) {
        self.interval = ((self.interval as f32 * MODIFIER).ceil() as usize).min(MAX_INTERVAL);
        self.date += chrono::Duration::days(self.interval as i64);
        println!("Updated: {self}")
    }

    pub fn repeat(&mut self) {
        self.date += chrono::Duration::days(self.interval as i64);
        println!("Repeated: {self}")
    }

    pub fn hard(&mut self) {
        self.interval = ((self.interval as f32 / MODIFIER).ceil() as usize).min(MAX_INTERVAL);
        self.date += chrono::Duration::days(self.interval as i64);
        println!("Hard updated {self}")
    }

    pub fn reset(&mut self) {
        self.interval = 1;
        self.date += chrono::Duration::days(self.interval as i64);
        println!("Reset {self}")
    }
}

impl std::fmt::Display for SpacedTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.name, self.date, self.interval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_entry_test() {
        let line = "guitar,2022-10-01,1d";
        let want = SpacedTask {
            name: String::from("guitar"),
            date: Utc.ymd(2022, 10, 1).naive_utc(),
            interval: 1,
        };
        let got: SpacedTask = line.parse().unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn parse_schedule_test() {
        let pairs = vec![
            ("1d", Ok(1)),
            ("37D", Ok(37)),
            ("-1d", Err("Negative repeat interval")),
            ("d", Err("No number")),
            ("1", Err("Didn't get proper format. Expected <usize>d")),
        ];
        for (inp, want) in pairs {
            match (interval_days(inp), want) {
                (Ok(a), Ok(b)) => assert_eq!(a, b),
                (Err(a), Err(b)) => assert_eq!(format!("{}", a), format!("{}", b)),
                (Ok(a), Err(b)) => assert_eq!(format!("Ok: {}", a), format!("Err: {}", b)),
                (Err(a), Ok(b)) => assert_eq!(format!("Err: {}", a), format!("Ok: {}", b)),
            }
        }
    }
}
