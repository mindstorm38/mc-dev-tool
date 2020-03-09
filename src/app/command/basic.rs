use crate::command::{Command, CmdResult, CommandManager};

pub struct BasicCommand {
    help: &'static str,
    command: &'static str,
    callback: Box<dyn Fn(&[&str]) -> CmdResult>
}

impl Command for BasicCommand {

    fn get_command(&self) -> &'static str {
        self.command
    }

    fn get_help(&self) -> &'static str {
        self.help
    }

    fn execute(&mut self, args: &[&str], manager: &CommandManager) -> CmdResult {
        (self.callback)(args)
    }

}

impl BasicCommand {

    pub fn new<F>(help: &'static str, command: &'static str, callback: F) -> BasicCommand
        where F: 'static + (Fn(&[&str]) -> CmdResult) {

        BasicCommand {
            help,
            command,
            callback: Box::new(callback)
        }

    }

}