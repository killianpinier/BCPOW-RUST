pub mod error;
pub mod command_traits;

use std::{io::{self, Write}, str::SplitWhitespace};
use error::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct Instruction {
    command: String,
    flags: Vec<String>,
    args: Vec<String>,
}

impl Instruction {
    fn new() -> Instruction {
        Instruction { command: String::new(), flags: Vec::new(), args: Vec::new() }
    }
}

pub struct Cli {
    app_name: String,
    command_history: Vec<String>,
    history_size: u8,               // up to 255
}

impl Instruction {
    pub fn get_command(&self) -> &str {
        &self.command
    }

    pub fn get_arguments(&self) -> &Vec<String> {
        &self.args
    }

    pub fn get_flags(&self) -> &Vec<String> {
        &self.flags
    }
}

impl Cli {
    pub fn new(app_name: String) -> Cli {
        Cli { app_name, command_history: Vec::new(), history_size: 200 }
    }

    fn add_command_to_history(&mut self, command: &String) {
        if let Some(last) = self.command_history.last() {
            if *last != *command {
                self.command_history.push(command.to_string());
            }
        }
    }

    pub fn get_instruction(&mut self) -> Result<Option<Instruction>> {
        print!("{}> ", self.app_name);
        io::stdout().flush().map_err(Error::IoError)?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(Error::IoError)?;
        self.add_command_to_history(&input);

        Self::parse_instruction(input)
    }

    fn parse_instruction(input: String) -> Result<Option<Instruction>> {
        let mut iter = input.split_whitespace();
        let mut instruction = Instruction::new();

        match iter.next() {
            Some(word) => instruction.command = word.to_string(),
            None => return Ok(None),
        };

        Self::parse_flags_args(&mut instruction, &mut iter)?;
        Ok(Some(instruction))
    }

    fn parse_flags_args(instruction: &mut Instruction, iter: &mut SplitWhitespace) -> Result<()>{
        while let Some(word) = iter.next() {
            if word.starts_with('-') {
                if word.chars().filter(|&c| c == '-').count() > 2 {
                    return Err(Error::UnvalidFlagError(word.to_string()));
                }
                instruction.flags.push(word.to_string().replace("-", ""));
            } else {
                instruction.args.push(word.to_string())
            }
        }
        Ok(())
    }


}