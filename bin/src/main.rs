use file_handling;
use std::process;
use user_interface;

fn main() {
    let file_path = file_handling::get_file_path().unwrap_or_else(|error| {
        eprintln!("There was a problem getting the file path: {error}");
        process::exit(1);
    });

    let file = file_handling::open_file_or_create(&file_path).unwrap_or_else(|error| {
        eprintln!("There was a problem opening the file with the given file path: {error}");
        process::exit(1);
    });

    user_interface::display_main_menu();
    let selection = match user_interface::get_selection() {
        Ok(s) => s.parse_selection().unwrap_or_else(|error| {
            eprintln!("There was a problem parsing your selection: {error}");
            process::exit(1);
        }),
        Err(error) => {
            eprintln!("There was a problem receiving your selection: {error}");
            process::exit(1);
        }
    };
}
