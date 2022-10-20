use crate::spacedtask::SpacedTask;
use anyhow::{anyhow, Result};
use chrono::prelude::*;

pub fn try_add(tasks: &mut Vec<SpacedTask>, args: &[String]) -> Result<()> {
    let mut remaining = vec![];
    let mut next_is_date = false;
    let mut start = None;
    for a in &args[1..] {
        if next_is_date {
            start = Some(a.to_string());
            next_is_date = false;
        } else if a == "--start" || a == "-s" {
            next_is_date = true;
        } else {
            remaining.push(a.to_string());
        }
    }
    add(tasks, remaining.join(" "), start)
}

pub fn add_unscheduled(tasks: &mut Vec<SpacedTask>, title: String) -> Result<()> {
    let to_add = SpacedTask::new_unscheduled(title);

    tasks.push(to_add);
    Ok(())
}

pub fn add(tasks: &mut Vec<SpacedTask>, title: String, start: Option<String>) -> Result<()> {
    let date = match start {
        Some(date) => {
            let parts: Vec<&str> = date.split('-').collect();
            if parts.len() > 3 {
                return Err(anyhow!("Date should be in YYYY-MM-DD format"));
            }
            let yyyy = parts[0].parse()?;
            let mm = parts[1].parse()?;
            let dd = parts[2].parse()?;
            Some(Utc.ymd(yyyy, mm, dd).naive_utc())
        }
        None => None,
    };
    let to_add = SpacedTask::new(title, date);

    tasks.push(to_add);
    Ok(())
}

pub fn view(tasks: &[SpacedTask]) {
    for (i, t) in tasks.iter().enumerate() {
        println!("{i:4}. {t}");
    }
}

pub fn due(tasks: &[SpacedTask]) {
    let today = Utc::today().naive_utc();
    for (i, t) in tasks.iter().enumerate() {
        if t.date.is_some() && t.date.unwrap() <= today {
            println!("{i:4}. {t}");
        }
    }
}
