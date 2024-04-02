pub mod search;
pub mod install;
pub mod command_list;
pub mod run;

use std::error::Error;


pub trait Command {
    fn help(&self);
    fn run(&self) -> Result<(), Box<dyn Error>>;
    fn  set_from_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>>;
}