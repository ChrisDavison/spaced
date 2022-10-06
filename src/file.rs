use anyhow::{anyhow, Error, Result};
use chrono::prelude::*;

#[derive(Debug, PartialEq)]
pub struct SpacedTask {
    pub name: String,
    pub date: chrono::NaiveDate,
    pub interval: usize,
    pub modifier: f32,
}

pub fn parse() -> Vec<SpacedTask> {
    vec![]
}

impl std::str::FromStr for SpacedTask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").clone().collect();
        let name = parts[0];
        dbg!(&parts[1]);
        let dateparts = parts[1].split("-").collect::<Vec<&str>>();
        let date = NaiveDate::from_ymd(
            dateparts[0].parse()?,
            dateparts[1].parse()?,
            dateparts[2].parse()?,
        );
        let interval = interval_days(&parts[2])?;
        let modifier = parts[3].parse()?;
        Ok(SpacedTask {
            name: name.to_string(),
            date,
            interval,
            modifier,
        })
    }
}

fn interval_days(s: &str) -> Result<usize> {
    let mut cur_num = String::new();
    for ch in s.chars() {
        if ch.is_numeric() {
            cur_num.push(ch);
            continue;
        } else if ch == '-' {
            return Err(anyhow!("Negative repeat interval"));
        } else {
            if cur_num.is_empty() {
                return Err(anyhow!("No number"));
            }
            return cur_num
                .parse()
                .map_err(|_| anyhow!("Unrecognised number period {ch}"));
        }
    }
    Err(anyhow!("Didn't get proper format. Expected <usize>d"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_entry_test() {
        let line = "guitar,2022-10-01,1d,2.5";
        let want = SpacedTask {
            name: String::from("guitar"),
            date: Utc.ymd(2022, 10, 1).naive_utc(),
            interval: 1,
            modifier: 2.5,
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
