#![allow(dead_code, unused_variables, unused_imports)]
mod file;

#[macro_use]
extern crate lazy_static;

use file::SpacedTask;

use anyhow::Result;
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
        title: String,
    },
    /// Update a task one step
    Update {
        index: usize,
    },
    /// Repeat task using current step
    Repeat {
        index: usize,
    },
    /// Repeat task using the previous step
    Hard {
        index: usize,
    },
    /// View tasks
    View,
    Due,
}

fn main() {
    let args = Args::from_args();
    let filename = args.filename;
    let custom_intervals = match get_custom_intervals() {
        Ok(intervals) => intervals,
        Err(e) => {
            eprintln!("Error getting custom intervals from env var: {e}");
            return;
        }
    };
    dbg!(&custom_intervals);
    let mut tasks = match file::get_tasks(&filename) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    match args.cmd {
        Command::Add { title } => add(&mut tasks, title),
        Command::Hard { index } => tasks[index].reduce_interval(&custom_intervals),
        Command::Repeat { index } => tasks[index].repeat_interval(),
        Command::Update { index } => tasks[index].increase_interval(&custom_intervals),
        Command::View => view(&tasks),
        Command::Due => due(&tasks),
    }
    if let Err(e) = file::write_tasks(&tasks, &filename, &custom_intervals) {
        eprintln!("{e}");
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

fn add(tasks: &mut Vec<SpacedTask>, title: String) {
    let to_add = SpacedTask::new(title);
    tasks.push(to_add)
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
