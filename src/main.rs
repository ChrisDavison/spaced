#![allow(dead_code, unused_variables, unused_imports)]
mod file;

use file::SpacedTask;

use chrono::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "space task revisits")]
enum Command {
    /// Add a new task, due today
    Add { title: String },
    /// Update a task one step
    Update { index: usize },
    /// Repeat task using current step
    Repeat { index: usize },
    /// Repeat task using the previous step
    Hard { index: usize },
    /// View tasks
    View,
}

fn main() {
    let args = Args::from_args();
    let filename = "local.txt";
    let mut tasks = match file::get_tasks(filename) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    match args.cmd {
        Command::Add { title } => add(&mut tasks, title),
        Command::Hard { index } => tasks[index].hard(),
        Command::Repeat { index } => tasks[index].repeat(),
        Command::Update { index } => tasks[index].update(),
        Command::View => view(&tasks),
    }
    if let Err(e) = file::write_tasks(&tasks, filename) {
        eprintln!("{e}");
    }
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
