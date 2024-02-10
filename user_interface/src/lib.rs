use file_handling;
use std::fs;
use std::io;
use std::num::ParseIntError;

/// Selection type for main menu
pub struct Selection(String);

impl Selection {
    /// Parse user selection into an integer to use for choice handling
    pub fn parse_selection(&self) -> Result<i32, ParseIntError> {
        self.0.parse()
    }
}

pub enum TaskOperation {
    Add(Task),
    Remove(Task),
    View(Task),
}

#[derive(Debug)]
pub struct Task {
    pub contents: String,
    pub time_spent: String,
    pub date_completed: String,
}

/// Purpose: Display main selection menu to the user
pub fn display_main_menu() {
    println!("\n1. Add a completed task");
    println!("2. Remove a completed task");
    println!("3. View all completed tasks from a date range");
    println!("4. View all completed tasks of all time");
    println!("5. Exit\n");
}

pub fn get_selection() -> Result<Selection, io::Error> {
    let mut selection = Selection(String::new());
    io::stdin().read_line(&mut selection.0)?;
    selection.0 = selection.0.trim().to_string();
    Ok(selection)
}

pub fn handle_choice(file: &fs::File, choice: &i32) -> Result<(), io::Error> {
    match *choice {
        1 => match add_task() {
            Ok(TaskOperation::Add(task)) => {
                let _ = file_handling::write_to_file(file, &task.contents.as_bytes());
                Ok(())
            }
            Err(e) => Err(e),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid Input")),
        },
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Temp: Need to implement other choices",
        )),
    }
}

fn add_task() -> Result<TaskOperation, io::Error> {
    let mut contents = String::new();
    let mut time_spent = String::new();
    let mut date_completed = String::new();

    println!("Enter the task name: ");
    io::stdin().read_line(&mut contents)?;

    println!("Enter the time you spent on the task (minutes): ");
    io::stdin().read_line(&mut time_spent)?;

    println!("Enter the date you completed the task: ");
    io::stdin().read_line(&mut date_completed)?;

    Ok(TaskOperation::Add(Task {
        contents: contents.trim().to_string(),
        time_spent: time_spent.trim().to_string(),
        date_completed: date_completed.trim().to_string(),
    }))
}
fn remove_task() {}
fn view_all_tasks() {}
fn view_all_tasks_in_range() {}

#[cfg(test)]
mod tests {
    use super::*;

    // These unit tests won't work because they require user input, check to see if there is a way to create
    // test user input so that we can write these tests. Such as maybe passing a writer or using io::Cursor, or handling
    // user input in the main binary
    #[test]
    fn user_selection() {}

    #[test]
    fn handle_add_choice() {}
}
