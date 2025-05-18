mod models;
mod util;

// gonna do REPL instead of cli because better ux!
use models::{Task, TaskJson};
use std::io::{self, Write};

const TASK_JSON_FILE: &str = "task.json";

fn main() {
    let mut task_json = TaskJson::from_file(TASK_JSON_FILE);

    // why not :3
    // let len = u32::MAX.to_string().len() + "mark-in-progress".len() + 3 + 50
    let mut buffer = String::with_capacity(79);
    let stdin = io::stdin();

    println!("+++++ task-tracker 0.1.0 +++++");
    println!("+++++ Type ':h' for help +++++");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();

        let args: Vec<&str> = buffer.trim().split_whitespace().collect();
        let args_len = args.len();

        if args_len == 0 {
            continue;
        }

        match args[0] {
            ":h" => {
                println!("\nAvailable commands:");
                println!("  :h                             Show this help message");
                println!("  :a  [content]                  Add a task");
                println!("  :uc [id] [content]             Update a task content");
                println!(
                    "  :us [id] [status]              Update a task status (todo, done, inprogress)"
                );
                println!("  :d  [id]                       Delete a task");
                println!("  :l  [status]                   List tasks (optional filter: status)");
                println!("  :s  [id]                       Show task detail");
                println!("  :q                             Quit program");
            }
            ":q" => {
                std::process::exit(0);
            }
            ":a" => {
                if args_len < 2 {
                    println!("Missing argument: content");
                    continue;
                }

                let id = task_json.insert_task(Task::new(args[1]));
                task_json.save_to_file(TASK_JSON_FILE);
                println!("Added task. Task id: {}", id);
            }
            ":uc" => {
                if args_len < 2 {
                    println!("Missing argument: id, content");
                    continue;
                } else if args_len < 3 {
                    println!("Missing argument: content");
                    continue;
                }

                let id = match args[1].parse::<u32>() {
                    Ok(v) => v,
                    _ => {
                        println!("Id must be a valid u32 number.");
                        continue;
                    }
                };

                if !task_json.update_task(id, Some(args[2]), None).is_some() {
                    println!("Task with id {} doesn't exist!", id);
                    continue;
                };

                task_json.save_to_file(TASK_JSON_FILE);
                println!("Updated task.");
            }
            ":us" => {
                if args_len < 2 {
                    println!("Missing argument: id, status");
                    continue;
                } else if args_len < 3 {
                    println!("Missing argument: status");
                    continue;
                }

                let id = match args[1].parse::<u32>() {
                    Ok(v) => v,
                    _ => {
                        println!("Id must be a valid u32 number.");
                        continue;
                    }
                };

                let status = if !["todo", "done", "inprogress"].contains(&args[2]) {
                    println!("Please enter a valid status.");
                    continue;
                } else {
                    args[2]
                };

                if !task_json.update_task(id, None, Some(status)).is_some() {
                    println!("Task with id {} doesn't exist!", id);
                    continue;
                };

                task_json.save_to_file(TASK_JSON_FILE);
                println!("Updated task.");
            }
            ":d" => {
                if args_len < 2 {
                    println!("Missing argument: id");
                    continue;
                }

                let id = match args[1].parse::<u32>() {
                    Ok(v) => v,
                    _ => {
                        println!("Id must be a valid u32 number.");
                        continue;
                    }
                };

                if !task_json.remove_task(id).is_some() {
                    println!("Task with id {} doesn't exist!", id);
                    continue;
                }

                task_json.save_to_file(TASK_JSON_FILE);
                println!("Deleted task.");
            }
            ":l" => {
                if args_len == 1 {
                    println!("{:?}", task_json.get_id_description_pair());
                } else if args_len >= 2 {
                    let status = if !["todo", "done", "inprogress"].contains(&args[1]) {
                        println!("Please enter a valid status.");
                        continue;
                    } else {
                        args[1]
                    };

                    println!("{:?}", task_json.get_id_description_pair_filtered(status));
                }
            }
            ":s" => {
                if args_len < 2 {
                    println!("Missing argument: id");
                    continue;
                }

                let id = match args[1].parse::<u32>() {
                    Ok(v) => v,
                    _ => {
                        println!("Id must be a valid u32 number.");
                        continue;
                    }
                };

                if let Some(task) = task_json.get_task(id) {
                    println!("{:#?}", task);
                } else {
                    println!("Task doesn't exist");
                }
            }
            _ => {
                println!("Unknown command. Type ':h' for help.");
            }
        }
        println!("");
    }
}
