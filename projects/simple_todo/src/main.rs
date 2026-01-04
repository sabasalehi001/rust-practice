use std::io::{self, BufRead, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn load_tasks(filepath: &str) -> Vec<Task> {
    let path = Path::new(filepath);
    if !path.exists() {
        return Vec::new();
    }

    let file = File::open(filepath).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);

    let mut tasks = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        match serde_json::from_str::<Task>(&line) {
            Ok(task) => tasks.push(task),
            Err(e) => println!("Error parsing task: {:?}, skipping line", e),
        }
    }
    tasks
}

fn save_tasks(filepath: &str, tasks: &Vec<Task>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filepath)?;
    let mut writer = std::io::BufWriter::new(file);

    for task in tasks {
        let json_string = serde_json::to_string(task).unwrap();
        writeln!(&mut writer, "{}", json_string)?;
    }
    Ok(())
}

fn add_task(tasks: &mut Vec<Task>) {
    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");
    let description = description.trim().to_string();
    tasks.push(Task { description, completed: false });
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks in the list.");
        return;
    }

    for (i, task) in tasks.iter().enumerate() {
        println!("{}: [{}] {}", i + 1, if task.completed { "x" } else { " " }, task.description);
    }
}

fn complete_task(tasks: &mut Vec<Task>) {
    print!("Enter task number to complete: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    if index > 0 && index <= tasks.len() {
        tasks[index - 1].completed = true;
    } else {
        println!("Invalid task number.");
    }
}

fn remove_task(tasks: &mut Vec<Task>) {
    print!("Enter task number to remove: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    if index > 0 && index <= tasks.len() {
        tasks.remove(index - 1);
    } else {
        println!("Invalid task number.");
    }
}

fn main() {
    let filepath = "tasks.json";
    let mut tasks = load_tasks(filepath);

    loop {
        println!("\nOptions:");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Complete task");
        println!("4. Remove task");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => complete_task(&mut tasks),
            "4" => remove_task(&mut tasks),
            "5" => break,
            _ => println!("Invalid choice."),
        }

        save_tasks(filepath, &tasks).expect("Unable to save tasks");
    }

    println!("Goodbye!");
}
