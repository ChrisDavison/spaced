#![allow(dead_code, unused_variables, unused_imports)]
mod commands;
mod file;
mod spacedtask;
mod util;

#[macro_use]
extern crate lazy_static;

use commands::*;
use spacedtask::SpacedTask;
use util::get_custom_intervals;

use anyhow::{anyhow, Result};
use chrono::prelude::*;

const USAGE: &str = "spaced vVERSION_FROM_CARGO

usage:
    spaced <filename> <command> <command_args>

commands:
    a|add    [-s|--start <YYYY-MM-DD>] <TITLE>...
        Add a task with title <TITLE...>
        optionally starting on date `--start`

    l|log   <TITLE>...
        Add a task with title <TITLE>..., with no date

    u|update <IDX>
        Update task using a harder step

    s|schedule <IDX>
        Add a date to an unscheduled task.

    r|repeat <IDX>
        Update task using the current step

    h|hard   <IDX>
        Update task using an easier step

    v|view
        View all tasks

    d|due
        Display tasks due today, or overdue";

fn usage(code: i32) {
    println!(
        "{}",
        USAGE.replace("VERSION_FROM_CARGO", env!("CARGO_PKG_VERSION"))
    );
    std::process::exit(code);
}

fn try_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        usage(0);
    }
    let filename = args[0].to_string();
    let args = &args[1..];
    let custom_intervals = get_custom_intervals()
        .map_err(|_| anyhow!("Couldn't get custom intervals from env var"))?;
    let mut tasks = file::get_tasks(&filename)?;

    match args.get(0).unwrap_or(&String::new()).as_ref() {
        "a" | "add" => try_add(&mut tasks, &args[1..])?,
        "l" | "log" => add_unscheduled(&mut tasks, args[1..].join(" "))?,
        "h" | "hard" => {
            let index: usize = args[1].parse()?;
            tasks[index].reduce_interval(&custom_intervals)
        }
        "r" | "repeat" => {
            let index: usize = args[1].parse()?;
            tasks[index].repeat_interval()
        }
        "u" | "update" | "s" | "schedule" => {
            let index: usize = args[1].parse()?;
            tasks[index].increase_interval(&custom_intervals)
        }
        "unscheduled" => {
            for (i, t) in tasks.iter().enumerate() {
                if t.date.is_none() {
                    println!("{i:4}. {t}");
                }
            }
        }
        "v" | "view" => view(&tasks),
        "d" | "due" => due(&tasks),
        _ => usage(0),
    }
    file::write_tasks(&tasks, &filename, &custom_intervals)
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
