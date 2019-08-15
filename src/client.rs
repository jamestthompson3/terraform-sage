use crate::cli::Command;
use crate::error::SageError;
use crate::terminal::{
    print_command_done,
    print_info,
    print_warning,
    print_error
};
use crate::utils::get_configs;


pub struct Client;


impl Client {

    pub fn new() -> Client {
        Client {}
    }

    pub fn run(&self, command: &Command) {
        let result = match command {
            Command::List {directory} => self.show_configurations(directory),
        };

        match result {
            Ok(_) => print_command_done(),
            Err(e) => print_error(e)
        }
    }

    fn show_configurations(&self, directory: &String) -> Result<(), SageError> {
        let configs = get_configs(directory)?;

        match configs.len() {
            count if count > 0 => {
                print_info("Available configurations:");
                configs
                    .keys()
                    .into_iter()
                    .for_each(|config| print_info(&format!("- {}", config)));
            },
            _ => print_warning("Configurations were not found."),
        };

        Ok(())
    }
}