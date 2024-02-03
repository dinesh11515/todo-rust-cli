use colored::*;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::{env, process};

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        let todo_path = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                let home: String = env::var("HOME").unwrap();
                let legacy_todo = format!("{}/TODO", &home);
                match Path::new(&legacy_todo).exists() {
                    true => legacy_todo,
                    false => format!("{}/.todo", &home),
                }
            }
        };
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect("Couldn't open the todofile");

        let mut buf_reader = BufReader::new(&todofile);

        // Empty String ready to be filled with TODOs
        let mut contents = String::new();

        // Loads "contents" string with data
        buf_reader.read_to_string(&mut contents).unwrap();
        let todo = contents.lines().map(str::to_string).collect();
        Ok(Self { todo, todo_path })
    }
    pub fn add(&self, args: &Vec<String>) {
        let todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist
            .append(true) // b) append a line to it
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);
        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            // Appends a new task/s to the file
            let line = format!("[ ] {}\n", arg);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
        print!("{:?}", buffer);
    }
    pub fn remove(&self, task: &String) {
        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");
        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if (line.contains(task)) {
                continue;
            }

            let nline = format!("{}\n", line);
            buffer
                .write_all(nline.as_bytes())
                .expect("can't able to write");
        }
    }
    pub fn done(&self, task: &String) {
        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");
        let mut buffer = BufWriter::new(&todofile);
        for (pos, line) in self.todo.iter().enumerate() {
            if (line.contains(task)) {
                let nline = format!("[*] {}\n", &line[4..]);
                buffer
                    .write_all(nline.as_bytes())
                    .expect("can't able to write");
            } else {
                let nline = format!("{}\n", line);
                buffer
                    .write_all(nline.as_bytes())
                    .expect("can't able to write");
            }
        }
    }

    pub fn edit(&self, oldtask: &String, newtask: &String) {
        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the file");
        let mut buffer = BufWriter::new(todofile);
        for (pos, line) in self.todo.iter().enumerate() {
            if line[4..].eq(oldtask) {
                let nline = format!("[ ] {}\n", newtask);
                buffer
                    .write_all(nline.as_bytes())
                    .expect("Can't able to write");
            } else {
                let nline = format!("{}\n", line);
                buffer
                    .write_all(nline.as_bytes())
                    .expect("Can't able to write");
            }
        }
    }

    pub fn list(&self) {
        let stdout = io::stdout();
        // Buffered writer for stdout stream
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();
        // This loop will repeat itself for each task in TODO file
        for (number, task) in self.todo.iter().enumerate() {
            if task.len() > 5 {
                // Converts virgin default number into a chad BOLD string
                let number = (number + 1).to_string();

                // Saves the symbol of current task
                let symbol = &task[..4];
                // Saves a task without a symbol
                let task = &task[4..];

                // Checks if the current task is completed or not...
                if symbol == "[*] " {
                    // DONE
                    // If the task is completed, then it prints it with a strikethrough
                    data = format!("{} {}\n", number, task.strikethrough());
                } else if symbol == "[ ] " {
                    // NOT DONE
                    // If the task is not completed yet, then it will print it as it is
                    data = format!("{} {}\n", number, task);
                }
                writer
                    .write_all(data.as_bytes())
                    .expect("Failed to write to stdout");
            }
        }
    }
}
