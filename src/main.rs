#![allow(dead_code, unused_variables, unused_imports)]
mod file;

#[macro_use]
extern crate lazy_static;

use file::SpacedTask;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(subcommand)]
    cmd: Command,
    filename: String,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "space task revisits")]
enum Command {
    /// Add a new task, due today
    Add {
        title: Vec<String>,
        /// Date of first repeat, in YYYY-MM-DD
        #[structopt(short, long)]
        start: Option<String>,
    },
    /// Update a task one step
    Update { index: usize },
    /// Repeat task using current step
    Repeat { index: usize },
    /// Repeat task using the previous step
    Hard { index: usize },
    /// View tasks
    View,
    /// List tasks due today, or overdue
    Due,
}

fn try_main(args: Args) -> Result<()> {
    let args = Args::from_args();
    let filename = args.filename;
    let custom_intervals = get_custom_intervals()
        .map_err(|_| anyhow!("Couldn't get custom intervals from env var"))?;
    let mut tasks = file::get_tasks(&filename)?;
    match args.cmd {
        Command::Add { title, start } => add(&mut tasks, title.join(" "), start)?,
        Command::Hard { index } => tasks[index].reduce_interval(&custom_intervals),
        Command::Repeat { index } => tasks[index].repeat_interval(),
        Command::Update { index } => tasks[index].increase_interval(&custom_intervals),
        Command::View => view(&tasks),
        Command::Due => due(&tasks),
    }
    file::write_tasks(&tasks, &filename, &custom_intervals)
}

fn main() {
    if let Err(e) = try_main(Args::from_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn get_custom_intervals() -> Result<Vec<usize>> {
    let mut intervals = Vec::new();
    if let Ok(interval_str) = std::env::var("SPACED_INTERVALS") {
        for thing in interval_str.split(',') {
            intervals.push(thing.trim().parse::<usize>()?);
        }
    }
    Ok(intervals)
}

fn add(tasks: &mut Vec<SpacedTask>, title: String, start: Option<String>) -> Result<()> {
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

fn view(tasks: &[SpacedTask]) {
    for (i, t) in tasks.iter().enumerate() {
        println!("{i:4}. {t}");
    }
}

fn due(tasks: &[SpacedTask]) {
    let today = Utc::today().naive_utc();
    for (i, t) in tasks.iter().enumerate() {
        if t.date <= today {
            println!("{i:4}. {t}");
        }
    }
}
