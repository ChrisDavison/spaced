use crate::spacedtask::SpacedTask;
use anyhow::{anyhow, Error, Result};
use chrono::prelude::*;
use std::path::Path;

pub fn get_tasks(filename: &str) -> Result<Vec<SpacedTask>> {
    let mut tasks = Vec::new();
    if !Path::exists(Path::new(filename)) {
        std::fs::File::create(filename)?;
    }
    let contents = std::fs::read_to_string(filename)?;
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        tasks.push(line.parse::<SpacedTask>()?);
    }
    Ok(tasks)
}

pub fn write_tasks(tasks: &[SpacedTask], filename: &str, custom_intervals: &[usize]) -> Result<()> {
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
        let mut name_parts = Vec::new();
        let mut date = None;
        let mut interval = None;
        let parts: Vec<&str> = s.split(' ').collect();
        for part in s.split(' ') {
            let mut ch = part.chars();
            match ch.next().unwrap() {
                '#' => {
                    if interval.is_some() {
                        return Err(anyhow!("Found a second `#...` interval element"));
                    }
                    interval = Some(part[1..].parse::<usize>()?)
                }
                '@' => {
                    if date.is_some() {
                        return Err(anyhow!("Found a second `@...` date element"));
                    }
                    let dateparts: Vec<&str> = part[1..].split('-').collect();
                    date = Some(NaiveDate::from_ymd(
                        dateparts[0].parse()?,
                        dateparts[1].parse()?,
                        dateparts[2].parse()?,
                    ));
                }
                _ => name_parts.push(part),
            }
        }
        let name = name_parts.join(" ");
        let interval = interval.ok_or_else(|| anyhow!("No interval in line. Need `@interval`"))?;
        Ok(SpacedTask {
            name,
            date,
            interval,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_entry_test() {
        let line = "guitar @2022-10-01 #1";
        let want = SpacedTask {
            name: String::from("guitar"),
            date: Utc.ymd(2022, 10, 1).naive_utc(),
            interval: 1,
        };
        let got: SpacedTask = line.parse().unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn parse_entry_with_comma_test() {
        let line = "guitar, i see fire (backwards) @2022-10-01 #1";
        let want = SpacedTask {
            name: String::from("guitar, i see fire (backwards)"),
            date: Utc.ymd(2022, 10, 1).naive_utc(),
            interval: 1,
        };
        let got: SpacedTask = line.parse().unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn increase_task() {
        let mut task = SpacedTask {
            name: String::from("guitar"),
            date: Utc.ymd(2022, 10, 1).naive_utc(),
            interval: 1,
        };
        task.increase_interval(&[]);
        assert_eq!(task.interval, 3);
        task.increase_interval(&[]);
        assert_eq!(task.interval, 8);
    }

    #[test]
    fn increase_task_with_custom_intervals() {
        let customs = vec![1, 7, 365, 720];
        let mut task = SpacedTask {
            name: String::from("guitar"),
            date: Utc.ymd(2022, 10, 1).naive_utc(),
            interval: 1,
        };
        task.increase_interval(&customs);
        assert_eq!(task.interval, 7);
        task.increase_interval(&customs);
        assert_eq!(task.interval, 365);
    }
}
