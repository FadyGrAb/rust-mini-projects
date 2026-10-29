use std::io::{self, Write};

enum Op {
    Add,
    Remove,
    Show,
    MarkComplete,
    Quit,
}

struct MenuItem {
    name: String,
    operation: Op,
}
impl MenuItem {
    fn new(name: &str, operation: Op) -> Self {
        Self {
            name: String::from(name),
            operation,
        }
    }
}

struct Task {
    description: String,
    completed: bool,
}
impl Task {
    fn new(description: String) -> Self {
        Self {
            description,
            completed: false,
        }
    }
}

fn main() {
    let options: Vec<MenuItem> = vec![
        MenuItem::new("Add", Op::Add),
        MenuItem::new("Remove", Op::Remove),
        MenuItem::new("Show", Op::Show),
        MenuItem::new("Mark Complete", Op::MarkComplete),
        MenuItem::new("Quit", Op::Quit),
    ];

    let mut tasks: Vec<Task> = Vec::new();

    'main_loop: loop {
        let mut input_buff = String::new();
        show_menu(&options);

        let _ = io::stdin().read_line(&mut input_buff);

        match input_buff.trim().parse::<usize>() {
            Ok(num) if num > 0 => match options.get(num - 1) {
                Some(item) => match item.operation {
                    Op::Add => {
                        show_prompt("Enter New Task");
                        match parse_input_text() {
                            Some(txt) => {
                                tasks.push(Task::new(txt));
                                println!("Task is added.");
                            }
                            None => println!("Empty entry. Nothing in added."),
                        }
                    }
                    Op::Remove => {
                        show_tasks(&tasks);
                        if !tasks.is_empty() {
                            show_prompt("Enter Task number");
                            match parse_input_text() {
                                Some(txt) => match txt.parse::<usize>() {
                                    Ok(idx) => {
                                        let task_index = idx - 1;
                                        if task_index < tasks.len() {
                                            tasks.remove(task_index);
                                            println!("Task is removed!")
                                        } else {
                                            println!("Index out of bound!")
                                        }
                                    }
                                    Err(_) => println!("Invalide number."),
                                },
                                None => println!("Invalid input!"),
                            }
                        }
                    }
                    Op::Show => show_tasks(&tasks),
                    Op::MarkComplete => {
                        show_tasks(&tasks);

                        if !tasks.is_empty() {
                            show_prompt("Enter task number");

                            match parse_input_text() {
                                Some(txt) => match txt.parse::<usize>() {
                                    Ok(idx) => match tasks.get_mut(idx - 1) {
                                        Some(task) => {
                                            task.completed = true;
                                            println!("Task is marked as Completed.\n")
                                        }
                                        None => println!("Index out of bound!"),
                                    },
                                    Err(_) => println!("Invalid number!"),
                                },
                                None => println!("Invalid Input!"),
                            }
                        }
                    }
                    Op::Quit => break 'main_loop,
                },
                None => println!("Choice out of bound!"),
            },
            _ => println!("Invalid input!\n"),
        }
    }
}

fn show_menu(options: &[MenuItem]) {
    println!("\nChoose one of the following options:");
    for (idx, item) in options.iter().enumerate() {
        println!("\t[{}] {}", idx + 1, item.name)
    }
    print!(">> ");
    io::stdout().flush().ok();
}

fn show_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("Tasks list is empty!");
        return;
    }

    println!("Current tasks:");
    for (idx, task) in tasks.iter().enumerate() {
        let status = if task.completed { "x" } else { " " };
        println!("  {} - {} [{}]", idx + 1, task.description, status)
    }
    println!()
}

fn parse_input_text() -> Option<String> {
    let mut input_buff = String::new();
    io::stdin().read_line(&mut input_buff).ok();
    let input_text = input_buff.trim();

    if input_text.is_empty() {
        return None;
    } else {
        return Some(input_text.to_string());
    }
}

fn show_prompt(message: &str) {
    print!("{message}: ");
    io::stdout().flush().ok();
}
