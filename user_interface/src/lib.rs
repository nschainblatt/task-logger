use std::io;
use std::num::ParseIntError;

/// Purpose: Display main selection menu to the user
pub fn display_main_menu() {
    println!("\n1. Add a completed task");
    println!("2. Remove a completed task");
    println!("3. View all completed tasks from a date range");
    println!("4. View all completed tasks of all time");
    println!("5. Exit\n");
}

/// Selection type for main menu
pub struct Selection(String);

impl Selection {
    /// Parse user selection into an integer to use for menu selection
    pub fn parse_selection(&self) -> Result<i32, ParseIntError> {
        self.0.parse()
    }
}

pub fn get_selection() -> Result<Selection, io::Error> {
    let mut selection = Selection(String::new());
    io::stdin().read_line(&mut selection.0)?;
    selection.0 = selection.0.trim().to_string();
    Ok(selection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_selection() {
        todo!("Look into passing reading and writing buffers to the get_selection function instead of the function \
        itself handling input, this will allow better unit tests");
    }
}
