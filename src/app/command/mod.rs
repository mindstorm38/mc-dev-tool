use std::collections::HashMap;
use ansi_term::Colour::{Yellow, Red};
use std::rc::Rc;

pub mod basic;
pub mod help;

pub type CmdResult = Result<(), String>;

pub trait Command {
    fn get_command(&self) -> &'static str;
    fn get_help(&self) -> &'static str;
    fn execute(&mut self, args: &[&str], manager: &CommandManager) -> CmdResult;
}

pub struct CommandManager {
    commands: HashMap<String, Box<dyn Command>>
}

impl CommandManager {

    pub fn new() -> CommandManager {
        CommandManager {
            commands: HashMap::new()
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {

        let key = String::from(command.get_command());

        if self.commands.contains_key(&key) {
            panic!("A command '{}' already exits.", &key);
        }

        self.commands.insert(key, command);

    }

    pub fn has_command(&self, name: &String) -> bool {
        return self.commands.contains_key(name);
    }

    pub fn get_command(&self, name: &String) -> Option<&Box<dyn Command>> {
        return self.commands.get(name);
    }

    pub fn show_help(&self) {

        println!("{}", Yellow.bold().paint("# List of available commands #"));

        for (_name, cmd) in &self.commands {
            print!("{}", Yellow.paint(format!("- {}", cmd.get_help())))
        }

    }

    pub fn execute(&mut self, input: &str) {

        let args = input.split_whitespace().collect::<Vec<&str>>();

        if args.len() > 0 {

            let cmd_name = String::from(args[0]);

            if let Some(cmd) = self.commands.get_mut(&cmd_name) {

                if let Err(msg) = cmd.execute(&args[1..], self) {
                    println!("{} {}", Red.bold().paint("Error :"), Red.paint(&msg));
                }

            } else {
                println!("{}{}{}", Red.paint("Invalid command '"), Yellow.paint(&cmd_name), Red.paint("'."));
            }

        }

    }

}