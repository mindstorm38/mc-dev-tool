use super::{Command, CommandManager, CmdResult};
use ansi_term::Colour::{Yellow, Red, Cyan};
use std::cell::RefCell;

pub struct HelpCommand();

impl Command for HelpCommand {

    fn get_command(&self) -> &'static str { "help" }

    fn get_help(&self) -> &'static str { "help [<command>]" }

    fn execute(&mut self, args: &[&str], manager: &CommandManager) -> CmdResult {

        if args.len() == 0 {
            manager.show_help();
        } else {

            let name = String::from(args[0]);

            if let Some(command) = manager.get_command(&name) {
                println!("{} {}", Yellow.paint("Usage :"), Cyan.paint(command.get_help()));
            } else {
                return Err(format!("{}{}{}", Red.paint("Invalid command '"), Yellow.paint(&name), Red.paint("'.")));
            }

        }

        Ok(())

    }

}

impl HelpCommand {

    pub fn new() -> HelpCommand {
        HelpCommand { }
    }

}