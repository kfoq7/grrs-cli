// use std::collections::HashMap;
mod tasks;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "grrs", about = "A command line to-do app written in rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

// #[derive(Debug)]
// struct Task {
//     task: String,
//     done: bool,
// }

// #[derive(Debug)]
// struct TaskList {
//     items: HashMap<String, Task>,
// }

fn main() {
    let opt = CommandLineArgs::from_args();
    println!("{:?}", opt);

    // let args = Cli::parse();

    // let mut list = TaskList {
    //     items: HashMap::new(),
    // };

    // list.items.insert(
    //     "learn".to_string(),
    //     Task {
    //         task: "Learn Rust".to_string(),
    //         done: false,
    //     },
    // );

    // list.items.insert(
    //     "cli".to_string(),
    //     Task {
    //         task: "Write CLI app".to_string(),
    //         done: false,
    //     },
    // );

    // if let Some(task) = list.items.get(&args.key) {
    //     println!("Task: {}, Done: {}", task.task, task.done)
    // } else {
    //     println!("Task with key `{}` not found", args.key)
    // }

    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
}
