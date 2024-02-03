use clap::Parser;
mod args;
mod todo;
use crate::todo::Todo;
use args::*;
use console::style;
use indicatif::ProgressBar;

fn main() {
    let todo: Todo = Todo::new().expect("err");

    // println!("{:?}", todo.todo_path);
    let args = Args::parse();

    // println!("This is {} neat", style("quite").cyan());
    // let bar = ProgressBar::new(1000);
    // print!("{:?}", bar.is_hidden());
    // for _ in 0..1000 {
    //     bar.inc(1);
    //     let x = 10;
    //     // ...
    // }
    // bar.finish();
    // for _ in 0..args.count {
    //     // println!("Hello {}!", args.name)
    // }

    match &args.command {
        Some(Commands::Add { task }) => todo.add(task),
        Some(Commands::List {}) => todo.list(),
        Some(Commands::Remove { task }) => todo.remove(task),
        Some(Commands::Done { task }) => todo.done(task),
        Some(Commands::Edit { old_task, new_task }) => todo.edit(old_task, new_task),
        None => todo.list(),
    }
}
