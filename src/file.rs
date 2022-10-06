use anyhow::{anyhow, Error, Result};
use chrono::prelude::*;

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

pub struct SpacedTask {
    pub name: String,
    pub date: chrono::DateTime<Utc>,
    pub interval: usize,
    pub modifier: f32,
}

pub fn parse() -> Vec<SpacedTask> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

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
